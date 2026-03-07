namespace csapitemplate.Api.Startup;

public partial class Startup
{
    private static void AddAuth(WebApplicationBuilder builder)
    {
        builder.Services.AddAuthentication();
        builder.Services.AddAuthorization();
    }
    
    private static void UseAuth(WebApplication application)
    {
        application.UseAuthorization();
        application.UseAuthentication();
    }
}