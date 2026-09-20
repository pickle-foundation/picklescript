# PickleScript Application Structure Standard

Version: 1.0

The compiler allows everything. The ecosystem recommends consistency.

==================================================
PROJECT ROOT
==================================================

Recommended:

```
my-project/

    src/
    tests/
    docs/
    examples/
    resources/
    migrations/
    scripts/
    public/
    pickle.toml
    README.md
    CHANGELOG.md
    LICENSE
```

==================================================
SOURCE STRUCTURE
==================================================

```
src/

    App.pkl

    Controllers/
    Models/
    Services/
    Repositories/
    Interfaces/
    Contracts/
    Enums/
    DTO/
    Requests/
    Responses/
    Middleware/
    Exceptions/
    Events/
    Listeners/
    Jobs/
    Database/
    Config/
    Utils/
```

==================================================
CLASSES
==================================================

Classes use PascalCase.

File name matches class name.

Example:

```
src/Models/User.pkl

class User {

}
```

Do not:

```
src/models/user.pkl

class user
```

==================================================
CONTROLLERS
==================================================

Folder:

`Controllers/`

Naming:

`<Name>Controller`

Examples:

- `UserController.pkl`
- `PackageController.pkl`
- `AuthController.pkl`

Responsibilities:

- Receive input
- Validate request
- Call services
- Return response

Avoid:

- SQL queries
- Business logic
- Large calculations

==================================================
SERVICES
==================================================

Folder:

`Services/`

Naming:

`<Name>Service`

Examples:

- `PaymentService.pkl`
- `PackageService.pkl`
- `AuthService.pkl`

Responsibilities:

- Business rules
- Complex operations
- Coordination between systems

==================================================
REPOSITORIES
==================================================

Folder:

`Repositories/`

Naming:

`<Name>Repository`

Examples:

- `UserRepository.pkl`
- `PackageRepository.pkl`

Responsibilities:

- Database access
- Storage queries
- Persistence

Example:

```
interface UserRepository {

    fn find(id: int) -> User?

    fn save(user: User)

}
```

```
class MariaUserRepository implements UserRepository {

}
```

==================================================
INTERFACES
==================================================

Folders:

`Interfaces/`

or

`Contracts/`

Naming:

Use capability names.

Good:

- Cacheable
- Serializable
- Authenticatable
- UserRepository

Avoid:

- IUserRepository

No Hungarian notation.

Interfaces describe behavior.

Example:

```
interface Cacheable {

    fn cacheKey() -> string

}
```

==================================================
ENUMS
==================================================

Folder:

`Enums/`

Naming:

PascalCase.

Examples:

- `UserStatus.pkl`
- `PackageState.pkl`
- `PaymentType.pkl`

Example:

```
enum UserStatus {

    Active

    Disabled

    Pending

}
```

With payloads:

```
enum Result {

    Success(value: string)

    Error(message: string)

}
```

Use enums instead of magic strings.

Bad:

```
if status == "active"
```

Good:

```
if status == UserStatus.Active
```

==================================================
MODELS
==================================================

Folder:

`Models/`

Represent domain entities.

Examples:

- User
- Package
- Version
- Repository

Models should not:

- Send emails
- Call APIs
- Handle HTTP

==================================================
DTOs
==================================================

Folder:

`DTO/`

Data transfer objects.

Naming:

`<Name>DTO`

Examples:

- CreateUserDTO
- PackagePublishDTO

Used for:

- API input
- API output
- Service boundaries

==================================================
REQUESTS
==================================================

Folder:

`Requests/`

Naming:

`<Name>Request`

Examples:

- LoginRequest
- CreatePackageRequest

Contains:

- Validation rules
- Input transformation

==================================================
RESPONSES
==================================================

Folder:

`Responses/`

Naming:

`<Name>Response`

Examples:

- UserResponse
- PackageResponse

Used for:

- API formatting

==================================================
MIDDLEWARE
==================================================

Folder:

`Middleware/`

Naming:

`<Name>Middleware`

Examples:

- AuthMiddleware
- RateLimitMiddleware

One responsibility.

==================================================
EXCEPTIONS
==================================================

Folder:

`Exceptions/`

Naming:

`<Name>Exception`

Examples:

- UserNotFoundException
- InvalidPackageException

==================================================
EVENTS
==================================================

Folder:

`Events/`

Naming:

`<Name>Event`

Examples:

- UserRegisteredEvent
- PackagePublishedEvent

==================================================
LISTENERS
==================================================

Folder:

`Listeners/`

Naming:

`<Name>Listener`

Examples:

- SendWelcomeEmailListener
- UpdatePackageStatsListener

==================================================
JOBS
==================================================

Folder:

`Jobs/`

Naming:

`<Name>Job`

Examples:

- SyncGithubRepositoryJob
- GenerateDocumentationJob

==================================================
DATABASE
==================================================

Folder:

`Database/`

Contains:

```
Database/

    Migrations/

    Seeders/

    Factories/
```

==================================================
UTILITY CLASSES
==================================================

Folder:

`Utils/`

Naming:

Purpose based.

Examples:

- StringHelper
- DateFormatter
- HashGenerator

Avoid:

- Helper.pkl
- Common.pkl
- Utils.pkl

==================================================
FUNCTION STANDARDS
==================================================

Functions:

camelCase

Good:

```
fn createPackage()
fn validateUser()
fn calculatePrice()
```

Avoid:

```
fn CreatePackage()
fn create_package()
```

==================================================
FUNCTION PARAMETERS
==================================================

Prefer descriptive names.

Good:

```
fn findUser(userId: int)
```

Bad:

```
fn find(id: int)
```

==================================================
BOOLEAN FUNCTIONS
==================================================

Use:

is, has, can, should

Examples:

```
fn isActive()
fn hasPermission()
fn canDelete()
```

==================================================
FACTORY FUNCTIONS
==================================================

Use:

create, make, from

Examples:

```
fn createUser()
fn fromJson()
fn makeConnection()
```

==================================================
STATIC FUNCTIONS
==================================================

Use sparingly.

Good:

Factory methods

Example:

```
User.fromJson()
```

Avoid:

Huge static utility classes.

==================================================
PRIVATE FUNCTIONS
==================================================

Private helpers:

camelCase

Example:

```
private fn validateEmail()
```

==================================================
CONSTANTS
==================================================

SCREAMING_SNAKE_CASE

Example:

```
const MAX_UPLOAD_SIZE
```

==================================================
GENERICS
==================================================

Generic names:

T, K, V

Examples:

```
class Repository<T>
class Map<K, V>
```

For domain generics:

```
class Result<TValue>
```

==================================================
TEST STRUCTURE
==================================================

```
tests/

    Unit/

    Feature/

    Integration/
```

Example:

```
tests/

  Feature/

    UserRegistrationTest.pkl

  Unit/

    PackageParserTest.pkl
```

==================================================
PACKAGE STRUCTURE
==================================================

A package should look like:

```
vendor/package/

src/

    Controllers/

    Services/

    Models/

tests/

docs/

examples/

pickle.toml

README.md
```

==================================================
GENERAL RULE
==================================================

| What | Case |
| --- | --- |
| Classes | PascalCase |
| Functions | camelCase |
| Variables | camelCase |
| Enums | PascalCase |
| Interfaces | PascalCase |
| Constants | SCREAMING_SNAKE_CASE |
| Files | Match primary declaration name |
| Folders | PascalCase for code namespaces |

The compiler allows everything. The ecosystem recommends consistency.