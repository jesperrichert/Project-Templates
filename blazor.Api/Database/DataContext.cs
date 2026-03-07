using csapitemplate.Api.Configuration;
using csapitemplate.Api.Database.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Internal;
using Microsoft.Extensions.Options;

namespace csapitemplate.Api.Database;

public class DataContext : DbContext
{
    public DbSet<BaseModel> BaseModels { get; set; }

    private readonly IOptions<DatabaseOptions> Options;

    public DataContext(IOptions<DatabaseOptions> options)
    {
        Options = options;
    }

    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        if (optionsBuilder.IsConfigured)
            return;

        optionsBuilder.UseNpgsql(
            $"Host={Options.Value.Host};" +
            $"Port={Options.Value.Port};" +
            $"Username={Options.Value.Username};" +
            $"Password={Options.Value.Password};" +
            $"Database={Options.Value.Database}"
        );
    }
}