using blazor.Api.Database.Interfaces;
using Microsoft.EntityFrameworkCore;

namespace blazor.Api.Database;

public class DatabaseRepository<T> where T : class
{
    private readonly DataContext DataContext;
    private readonly DbSet<T> Set;

    public DatabaseRepository(DataContext dataContext)
    {
        DataContext = dataContext;
        Set = DataContext.Set<T>();
    }

    public IQueryable<T> Query() => Set;

    public async Task<T> AddAsync(T entity)
    {
        if (entity is IActionTimestamps actionTimestamps)
        {
            actionTimestamps.CreatedAt = DateTimeOffset.UtcNow;
            actionTimestamps.UpdatedAt = DateTimeOffset.UtcNow;
        }
        
        var final = Set.Add(entity);
        await DataContext.SaveChangesAsync();
        return final.Entity;
    }

    public async Task UpdateAsync(T entity)
    {
        if (entity is IActionTimestamps actionTimestamps)
            actionTimestamps.UpdatedAt = DateTimeOffset.UtcNow;
        
        Set.Update(entity);
        await DataContext.SaveChangesAsync();
    }

    public async Task RemoveAsync(T entity)
    {
        Set.Remove(entity);
        await DataContext.SaveChangesAsync();
    }
}