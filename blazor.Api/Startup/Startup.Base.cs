using blazor.Shared.Http;
using Scalar.AspNetCore;

namespace blazor.Api.Startup;

public partial class Startup
{
    private static void AddBase(WebApplicationBuilder builder)
    {
        builder.Services.AddOpenApi();
        builder.Services.AddControllers().AddJsonOptions(options =>
        {
            options.JsonSerializerOptions.TypeInfoResolverChain.Add(SerializationContext.Default);
        });
        builder.Services.AddMemoryCache();
    }

    private static void UseBase(WebApplication application)
    {
        application.UseBlazorFrameworkFiles();
        application.UseStaticFiles();

        application.UseRouting();

        application.MapOpenApi();
        application.MapScalarApiReference("/api-docs");
    }

    private static void MapBase(WebApplication application)
    {
        application.MapControllers();
        application.MapFallbackToFile("index.html");
    }
}