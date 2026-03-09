namespace blazor.Shared.Http.Response;

public record PagedData<T>(T[] Data, int TotalLength);