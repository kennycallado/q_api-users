# q_api-base

## LOG * migration to sqlx

- install sqlx cli
``` bash
cargo install sqlx-cli --no-default-features --features postgres

# or just

cargo install sqlx-cli 
```

cada instrucción debe contener DATABASE_URL o existir en .env

- no estoy seguro si es necesario crear la base de datos
``` bash
DATABASE_URL="postgres://base:base@localhost/base" sqlx database create
# sqlx database drop
```

- prepare migrations
``` bash
sqlx migrate add --source ./src/database/migrations/ -r --timestamp <migration_name>

# then

sqlx migrate run --source ./src/database/migrations/
```

- prepare offline mode
``` bash
 cargo sqlx prepare
```

- update features

En general...
``` rust
// old
#[cfg(feature = "db")]
// new
#[cfg(feature = "db_diesel")]
```
En los que usen sqlx
``` rust
#[cfg(feature = "db_sqlx")]
```

## Changelog

### v0.1.5

1. Estructura de directorios

1. Renombrado módulo slides a module

1. Urls terminan en barra

## Init a repo

### Extender plantilla

Acceder al repositorio en github y pulsar **Use this template**, **Create a new repository**. Una vez creado el repositorio puedes clonarlo localmente y seguir los siguientes pasos:

1. 
   ``` bash
   git checkout -b base
   ```
1. 
   ``` bash
   git remote add base git@github.com:kennycallado/q_api-base.git
   git remote set-url base --no-push git@github.com:kennycallado/q_api-base.git
   ```
1. 
   ``` bash
   git branch --set-upstream-to=base/main
   ```
1. 
   ``` bash
   git pull
   ```

   ``` bash
   git pull base main --allow-unrelated-histories
   ```

<!-- quizá mejor si no mantiene la rama
1. 
   ``` bash
   git checkout main
   ```
1. 
   ``` bash
   git push origin base
   ```
-->

### Adaptar proyecto

Algunos ficheros deben ser revisados y actualizados para cada proyecto derivado de base. En general están listados en la siguiente lista.

#### Raíz del proyecto

- [ ] .env
  - Dirección de la base de datos
- [ ] Cargo.toml
  - Nombre del paquete
  - Revisar dependencias
  - Revisar features
- [ ] .neoconf.json
  - Revisar features
- [ ] Containerfile
  - Nombre del paquete desde Cargo.toml
- [ ] compose.yaml
  - Variables de entorno
  - Servicios extra
- [ ] Rocket.toml
  - Parámetros de configuración del proyecto
  - secret_key = `openssl rand -base64 32`

#### Directorio src

- [ ] Tests

#### Migraciones

Cada api tiene sus propias migraciones localizadas en el directorio `src/database/migrations`

#### Modules

Directorio principal de trabajo de cada api. Contendrá un módulo por cada entidad con la que trabaje la api y administrará sus rustas.

#### Module

Cada módulo deberá contener, `model.rs` y `controller.rs`. En caso de ser necesario el controlador puede ayudarse de un directorio `handlers` y el modelo puede tener un repositorio dentro del directorio `services`.

El directorio de servicios del módulo también puede contener por ejemplo, `helpers` para el controlador o implementación de `claims` para entidad user.

## Update a repo


## TODO:

- [ ] Something
- [X] New user add fcm row
- [X] New user add projects_api record
- [ ] Admin, coord and thera don't need to be assignet to a proyect...
- [ ] Why admin is unauthorized ?
