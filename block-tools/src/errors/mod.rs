use std::fmt;
mod castings;

#[derive(Debug, Clone)]
pub enum LoopError {
	GenericError,
	InternalError(InternalError),
	UserError(UserError),
	BlockError(BlockError),
}

impl fmt::Display for LoopError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			LoopError::GenericError => write!(f, "[g] Something unspecified went wrong."),
			LoopError::InternalError(err) => write!(f, "{}", err.to_string()),
			LoopError::UserError(err) => write!(f, "{}", err.to_string()),
			LoopError::BlockError(err) => write!(f, "{}", err.to_string()),
		}
	}
}

#[derive(Debug, Clone)]
pub enum BlockError {
	TypeExist(String),
	TypeGenericError(String),
	InputParse,
	/// Error for when a block method does not exist
	/// for a certain block type. (Name, Type)
	MethodExist(String, String),
}

impl fmt::Display for BlockError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			BlockError::TypeExist(name) => {
				write!(f, "[bte] A block type called '{}' was not found.", name)
			}
			BlockError::MethodExist(name, block_type) => {
				write!(
					f,
					"[bme] A method called '{}' was not found for the {} block type.",
					name, block_type
				)
			}
			BlockError::TypeGenericError(err) => {
				write!(f, "[btg] {}", err)
			}
			BlockError::InputParse => {
				write!(f, "[bip] The input string could not be parsed properly.")
			}
		}
	}
}

#[derive(Debug, Clone)]
pub enum UserError {
	PasswordTooShort,
	PasswordMatch,
	EmailConfirmError(EmailConfirmError),
	NameNonexist(String),
	NameConflict(String),
	NameTooShort(String),
	JwtGeneric,
	NoAccess(NoAccessSubject),
	NeedAuth,
	InsufficientFunds(i32),
}

impl fmt::Display for UserError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			UserError::NameConflict(name) => {
				write!(f, "[unc] Username '{}' is already in use.", name)
			}
			UserError::NameNonexist(name) => {
				write!(
					f,
					"[une] A user with the username '{}' was not found.",
					name
				)
			}
			UserError::PasswordTooShort => write!(f, "[ups] The password provided was too short."),
			UserError::NameTooShort(name) => {
				write!(f, "[uns] The username `{}` provided was too short.", name)
			}
			UserError::PasswordMatch => write!(f, "[upm] The password provided is not correct."),
			UserError::EmailConfirmError(err) => write!(f, "{}", err.to_string()),
			UserError::JwtGeneric => write!(
				f,
				"[ujg] Something unspecified went wrong with user sessions."
			),
			UserError::NoAccess(scope) => {
				write!(f, "[uad] Access to {} was denied.", scope.to_string())
			}
			UserError::NeedAuth => write!(f, "[uar] Authentication headers are required."),
			UserError::InsufficientFunds(needed) => write!(
				f,
				"[uci] Insuffecient funds. {} credits are needed.",
				needed
			),
		}
	}
}

#[derive(Debug, Clone)]
pub enum NoAccessSubject {
	DeleteBlock(i64),
	EditColor(i64),
	NotifBlock(i64),
	OtherUserCredits,
	UpdatePermissions(i64),
	ViewBlock(i64),
	ViewComment(i64),
}

impl fmt::Display for NoAccessSubject {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			NoAccessSubject::OtherUserCredits => write!(f, "another user's credits"),
			NoAccessSubject::UpdatePermissions(id) => {
				write!(f, "updating block {}'s permissions", id)
			}
			NoAccessSubject::DeleteBlock(id) => write!(f, "deleting block {}", id),
			NoAccessSubject::ViewBlock(id) => write!(f, "viewing block {}", id),
			NoAccessSubject::ViewComment(id) => write!(f, "viewing comment {}", id),
			NoAccessSubject::NotifBlock(id) => write!(f, "setting block {}'s notifications", id),
			NoAccessSubject::EditColor(id) => write!(f, "changing block {}'s color", id),
		}
	}
}

#[derive(Debug, Clone)]
pub enum EmailConfirmError {
	NotFound(String),
	Expired,
	Invalid,
}

impl fmt::Display for EmailConfirmError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			EmailConfirmError::NotFound(name) => write!(
				f,
				"[uecn] An email confirmation with the username \"{}\" was not found.",
				name
			),
			EmailConfirmError::Invalid => write!(
				f,
				"[ueci] The verification code and/or session code were incorrect.",
			),
			EmailConfirmError::Expired => write!(
				f,
				"[uece] The email confirmation has expired because more than 5 minutes has passed since its creation.",
			),
		}
	}
}

#[derive(Debug, Clone)]
pub enum InternalError {
	DatabaseTimeout,
	GenericInternalError,
	EmailError,
}

impl fmt::Display for InternalError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			InternalError::DatabaseTimeout => write!(
				f,
				"[idt] There was an issue with connecting to the database."
			),
			InternalError::GenericInternalError => {
				write!(f, "[ig] Something unspecified went wrong internally.",)
			}
			InternalError::EmailError => {
				write!(f, "[img] Something went wrong with Loop's emailing system.")
			}
		}
	}
}
