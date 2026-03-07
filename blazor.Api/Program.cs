namespace csapitemplate.Api;

public static class Program
{
    public static async Task Main(String[] args)
    {
        var builder = WebApplication.CreateSlimBuilder(args);
        var startup = new Startup.Startup();

        await startup.PerformPreBuildAsync(builder);

        var application = builder.Build();

        await startup.PerformPostBuildAsync(application);

        await application.RunAsync();
    }
}