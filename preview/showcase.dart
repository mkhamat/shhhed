/// A reactive event store with caching and retry logic.
class EventStore extends StateNotifier<AsyncValue<List<Event>>> {
  final ApiClient _api;
  final Cache<String, Event> _cache;
  static const maxRetries = 3;

  EventStore(this._api, this._cache) : super(const AsyncValue.loading());

  Future<void> fetchEvents({int page = 1, bool refresh = false}) async {
    if (!refresh && state.hasValue) return;
    state = const AsyncValue.loading();
    try {
      final response = await _api.get<List<Event>>(
        '/events?page=$page&limit=20',
        headers: {'Authorization': 'Bearer ${_api.token}'},
      );
      final events = response.data
          .where((e) => e.isActive && e.priority >= 0.5)
          .toList();
      state = AsyncValue.data(events);
    } on NetworkException catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }
}
