using blazor.Api.Configuration;
using blazor.Api.Database;

namespace blazor.Api.Startup;

public partial class Startup
{
    private static void AddDatabase(WebApplicationBuilder builder)
    {
        builder.Services.AddOptions<DatabaseOptions>().BindConfiguration("blazor:Database");
        
        builder.Services.AddScoped(typeof(DatabaseRepository<>));
        builder.Services.AddDbContext<DataContext>();
    }
}