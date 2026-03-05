"""Streaming pipeline with backpressure and graceful shutdown."""
from dataclasses import dataclass, field
from typing import AsyncIterator, Callable
import asyncio

BATCH_SIZE = 256

@dataclass
class Pipeline:
    name: str
    buffer_limit: int = 4096
    _queue: asyncio.Queue = field(default_factory=asyncio.Queue)
    _running: bool = False

    async def start(self, source: AsyncIterator[dict]) -> None:
        """Consume from source with backpressure control."""
        self._running = True
        async for batch in self._chunked(source, size=BATCH_SIZE):
            if not self._running:
                break
            transformed = [
                item | {"pipeline": self.name, "ts": asyncio.get_event_loop().time()}
                for item in batch
                if item.get("valid", False) and item["score"] >= 0.75
            ]
            await self._queue.put(transformed)
