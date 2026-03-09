using Microsoft.AspNetCore.Identity;

namespace blazor.Api.Startup;

public static partial class Startup
{
    public static void PerformPreBuild(this WebApplicationBuilder builder)
    {
        AddBase(builder);
        AddAuth(builder);
        AddDatabase(builder);
        AddConfig(builder);
    }

    public static void PerformPostBuild(this WebApplication application)
    {
        UseBase(application);
        UseAuth(application);
        MapBase(application);
    }
}