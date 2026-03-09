using System.Diagnostics.CodeAnalysis;
using blazor.Api.Database.Entities;
using blazor.Shared.Http.Request.User;
using blazor.Shared.Http.Response.User;
using Riok.Mapperly.Abstractions;

namespace blazor.Api.Mapper;

[Mapper]
[SuppressMessage("Mapper", "RMG020:No members are mapped in an object mapping")]
[SuppressMessage("Mapper", "RMG012:No members are mapped in an object mapping")]
public static partial class UserMapper
{
    public static partial IQueryable<UserDto> ProjectToDto(this IQueryable<User> users);
    
    
    public static partial void Merge([MappingTarget] User user, UpdateUserDto request);

    
    public static partial UserDto ToDto(User user);
    
    public static partial User ToEntity(CreateUserDto request);
}