using Scalar.AspNetCore;

namespace csapitemplate.Api.Startup;

public partial class Startup
{
    private static void AddOpenApi(WebApplicationBuilder builder)
    {
        builder.Services.AddOpenApi();
    }

    private static void UseOpenApi(WebApplication application)
    {
        application.MapOpenApi();
        application.MapScalarApiReference("/api-docs");
    }
}