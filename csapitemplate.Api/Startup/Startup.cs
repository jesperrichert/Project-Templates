using Microsoft.AspNetCore.Identity;

namespace csapitemplate.Api.Startup;

public partial class Startup
{
    public  async Task PerformPreBuildAsync(WebApplicationBuilder builder)
    {
        AddOpenApi(builder);
        AddAuth(builder);
    }

    public async Task PerformPostBuildAsync(WebApplication application)
    {
        UseOpenApi(application);
        UseAuth(application);
    }
}