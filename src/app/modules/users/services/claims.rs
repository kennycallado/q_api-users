use crate::app::providers::constants::ACCESS_TOKEN_EXPIRATION;
use crate::app::providers::services::claims::{Claims, RoleInClaims, UserInClaims};

use crate::app::modules::users::model::UserExpanded;

impl From<UserExpanded> for UserInClaims {
    fn from(user: UserExpanded) -> Self {
        UserInClaims {
            id: user.id,
            depends_on: user.depends_on.id,
            role: RoleInClaims {
                id: user.role.id,
                name: user.role.name,
            },
            user_token: user.user_token,
        }
    }
}

impl From<UserExpanded> for Claims {
    fn from(user: UserExpanded) -> Self {
        let iat = chrono::Utc::now().timestamp();
        let exp = iat + ACCESS_TOKEN_EXPIRATION;

        let user_in_claims = UserInClaims {
            id: user.id,
            depends_on: user.depends_on.id,
            role: RoleInClaims {
                id: user.role.id,
                name: user.role.name,
            },
            user_token: user.user_token,
        };

        Claims {
            sub: user.id.to_string(),
            user: user_in_claims,
            iat,
            exp,
        }
    }
}
