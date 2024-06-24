use crate::lexer::{Token, TokenType};

pub enum ASTNodeType {
	Assignment,
	UnaryExpr,
	BinaryExpr,
}

pub enum Operator {
	Plus,
	Minus,
	Multiply,
	Divide,
	Modulo
}

pub struct ASTNode {
	pub left: Token,
	pub right: Option<Box<ASTNode>>,
	pub op: Operator,
	pub node_type: ASTNodeType,
}
