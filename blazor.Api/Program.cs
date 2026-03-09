using blazor.Api.Startup;

namespace blazor.Api;

public static class Program
{
    public static async Task Main(String[] args)
    {
        var builder = WebApplication.CreateBuilder(args);

        builder.PerformPreBuild();

        var application = builder.Build();

        application.PerformPostBuild();

        await application.RunAsync();
    }
}