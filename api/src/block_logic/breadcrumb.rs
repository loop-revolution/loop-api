use async_graphql::*;
use block_tools::use_diesel::prelude::*;
use block_tools::{
	blocks::Context,
	models::{Block, Property},
	schema::properties,
	Error,
};
use block_types::{blocks::group_block, delegation::display::delegate_block_name};

#[derive(SimpleObject, Clone)]
pub struct BreadCrumb {
	pub block_id: i64,
	pub name: String,
}

pub fn gen_breadcrumb(context: &Context, block: &Block) -> Result<Vec<BreadCrumb>, Error> {
	let mut crumbs = cycle(context, block, vec![], vec![])?.0;
	crumbs.reverse();
	Ok(crumbs)
}

fn cycle(
	context: &Context,
	block: &Block,
	mut crumbs: Vec<BreadCrumb>,
	mut blocks_added: Vec<i64>,
) -> Result<(Vec<BreadCrumb>, Vec<i64>), Error> {
	let conn = &context.pool.get()?;
	let parent_props: Vec<Property> = properties::dsl::properties
		.filter(properties::value_id.eq(block.id))
		.get_results(conn)?
		.into_iter()
		.filter(|prop: &Property| !blocks_added.contains(&prop.parent_id))
		.collect();
	if parent_props.len() == 0 {
		crumbs.push(BreadCrumb {
			block_id: block.id,
			name: delegate_block_name(context, &block.block_type, block)?,
		});
		return Ok((crumbs, blocks_added));
	}
	let parent_prop = &parent_props[0];
	let parent = Block::by_id(parent_prop.parent_id, conn)?;
	let parent = match parent {
		Some(parent) => parent,
		None => return Ok((crumbs, blocks_added)),
	};
	if parent.block_type == group_block::BLOCK_NAME {
		crumbs.push(BreadCrumb {
			block_id: block.id,
			name: delegate_block_name(context, &block.block_type, block)?,
		})
	} else {
		crumbs.push(BreadCrumb {
			block_id: block.id,
			name: parent_prop.property_name.clone(),
		})
	}
	blocks_added.push(block.id);
	let next_cycle = cycle(context, &parent, crumbs, blocks_added)?;
	Ok((next_cycle.0, next_cycle.1))
}