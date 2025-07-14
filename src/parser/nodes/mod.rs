pub mod number_node;
pub mod bin_op_node;
pub mod assignment_node;
pub mod variable_node;
pub mod expression_node;
pub mod echo_node;
pub mod scopre_node;
pub mod function_node;

pub use number_node::numberNode;
pub use bin_op_node::binOpNode;
pub use assignment_node::assignmentNode;
pub use variable_node::variableNode;
pub use expression_node::expressionNode;
pub use echo_node::echoNode;
pub use scopre_node::scopeNode;
pub use function_node::functionNode;