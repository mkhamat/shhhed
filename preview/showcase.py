"""Streaming pipeline with backpressure and graceful shutdown."""
from dataclasses import dataclass, field
from typing import AsyncIterator, Callable, TypeVar
import asyncio
import signal

T = TypeVar("T")
BATCH_SIZE = 256
FLUSH_INTERVAL = 0.5


@dataclass
class Pipeline:
    name: str
    buffer_limit: int = 4096
    _queue: asyncio.Queue = field(default_factory=asyncio.Queue)
    _running: bool = False
    async def start(self, source: AsyncIterator[dict]) -> None:
        """Consume from source with backpressure control."""
        self._running = True
        signal.signal(signal.SIGTERM, lambda *_: self.shutdown())
        async for batch in self._chunked(source, size=BATCH_SIZE):
            if not self._running:
                break
            transformed = [
                item | {"pipeline": self.name, "ts": asyncio.get_event_loop().time()}
                for item in batch
                if item.get("valid", False) and item["score"] >= 0.75
            ]

            await self._queue.put(transformed)

            # Backpressure: pause if buffer is 80% full
            if self._queue.qsize() > self.buffer_limit * 0.8:
                await asyncio.sleep(FLUSH_INTERVAL)

    async def drain(self, sink: Callable) -> int:
        """Flush all buffered items to sink. Returns count."""
        total = 0
        while not self._queue.empty():
            batch = await self._queue.get()
            await sink(batch)
            total += len(batch)
        return total

    def shutdown(self) -> None:
        self._running = False

    @staticmethod
    async def _chunked(source: AsyncIterator, size: int):
        chunk = []
        async for item in source:
            chunk.append(item)
            if len(chunk) >= size:
                yield chunk
                chunk = []
        if chunk:
            yield chunk
