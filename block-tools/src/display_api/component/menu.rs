use serde::Serialize;

use crate::{
	auth::permissions::{has_perm_level, PermLevel},
	models::Block,
};

#[derive(Serialize, Debug)]
pub struct MenuComponent {
	pub block_id: i64,
	pub star_button: Option<StarButton>,
	pub notifications_enabled: Option<bool>,
	pub delete: Option<bool>,
	pub permissions: Option<PermissionsList>,
}

#[derive(Serialize, Debug)]
pub struct StarButton {
	pub starred: bool,
	pub count: usize,
}

#[derive(Serialize, Debug)]
pub struct PermissionsList {
	pub full: usize,
	pub edit: usize,
	pub view: usize,
	pub public: Option<bool>,
}

impl MenuComponent {
	pub fn new(block_id: i64) -> Self {
		Self {
			block_id: block_id,
			notifications_enabled: None,
			delete: None,
			permissions: None,
			star_button: None,
		}
	}

	pub fn load_from_block(block: &Block, user_id: i32) -> Self {
		let mut menu = MenuComponent::new(block.id);

		if has_perm_level(user_id, &block, PermLevel::View) {
			menu.notifications_enabled = Some(block.notif_enabled.contains(&user_id));
			menu.star_button = Some(StarButton {
				count: block.stars.len(),
				starred: block.stars.contains(&user_id),
			});
			let public = if has_perm_level(user_id, &block, PermLevel::Full) {
				Some(block.public)
			} else {
				None
			};
			menu.permissions = Some(PermissionsList {
				public,
				full: block.perm_full.len(),
				edit: block.perm_edit.len(),
				view: block.perm_view.len(),
			});
		}

		if has_perm_level(user_id, &block, PermLevel::Owner) {
			menu.delete = Some(true);
		}

		menu
	}
}
