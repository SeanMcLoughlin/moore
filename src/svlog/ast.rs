// Copyright (c) 2016-2017 Fabian Schuiki

use source::Span;
use name::Name;

pub use self::TypeData::*;
pub use self::PortKind::*;
pub use self::StmtData::*;
pub use self::ExprData::*;


#[derive(Debug)]
pub struct ModDecl {
	pub span: Span,
	pub lifetime: Lifetime, // default static
	pub name: Name,
	pub name_span: Span,
	pub ports: Vec<Port>,
}

#[derive(Debug)]
pub struct IntfDecl {
	pub span: Span,
	pub lifetime: Lifetime, // default static
	pub name: Name,
	pub name_span: Span,
	pub ports: Vec<Port>,
}

#[derive(Debug)]
pub struct PackageDecl {
	pub span: Span,
	pub lifetime: Lifetime,
	pub name: Name,
	pub name_span: Span,
	pub timeunits: Timeunit,
	pub items: Vec<HierarchyItem>,
}



/// Lifetime specifier for variables, tasks, and functions. Defaults to static.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lifetime {
	Static,
	Automatic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timeunit;



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyItem;



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
	pub span: Span,
	pub data: TypeData,
	pub sign: TypeSign,
	pub dims: Vec<TypeDim>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeData {
	ImplicitType,
	VoidType,
	NamedType(Name),

	// Scoping
	ScopedType {
		ty: Box<Type>,
		member: bool,
		name: Name,
		name_span: Span,
	},

	// Integer Vector Types
	BitType,
	LogicType,
	RegType,

	// Integer Atom Types
	ByteType,
	ShortIntType,
	IntType,
	LongIntType,
	TimeType,

	// Non-integer Types
	ShortRealType,
	RealType,
	RealtimeType,

	// Enumerations
	EnumType(Option<Box<Type>>, Vec<EnumName>),
	StructType {
		kind: StructKind,
		packed: bool,
		signing: TypeSign,
		members: Vec<StructMember>,
	},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeSign {
	None,
	Signed,
	Unsigned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDim {
	Expr,
	Range,
	Queue,
	Unsized,
	Associative,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumName {
	pub span: Span,
	pub name: Name,
	pub name_span: Span,
	pub range: Option<Expr>,
	pub value: Option<Expr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructKind {
	Struct,
	Union,
	TaggedUnion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructMember {
	pub span: Span,
	pub rand_qualifier: Option<RandomQualifier>,
	pub ty: Box<Type>,
	pub names: Vec<VarDeclName>,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port {
	pub span: Span,
	pub name: Name,
	pub name_span: Span,
	// If kind, type, direction all omitted, inherit from previous port.
	pub kind: PortKind, // input,inout => net, output w. impl. type => net, output w. expl. type => var, ref => var
	pub ty: Type, // default logic
	pub dir: PortDir, // inherit or default inout if first
	pub dims: Vec<TypeDim>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortDecl {
	pub span: Span,
	pub dir: PortDir,
	pub net_type: Option<NetType>,
	pub var: bool,
	pub ty: Type,
	pub names: Vec<VarDeclName>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortKind {
	NetPort,
	VarPort,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortDir {
	Input,
	Output,
	Inout,
	Ref,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetType {
	Supply0,
	Supply1,
	Tri,
	TriAnd,
	TriOr,
	TriReg,
	Tri0,
	Tri1,
	Uwire,
	Wire,
	WireAnd,
	WireOr,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamPort {
	pub span: Span,
	pub name: Name,
	pub name_span: Span,
	pub ty: Type,
	pub dims: Vec<TypeDim>,
	pub init: (),
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Procedure {
	pub span: Span,
	pub kind: ProcedureKind,
	pub stmt: Stmt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcedureKind {
	Initial,
	Always,
	AlwaysComb,
	AlwaysLatch,
	AlwaysFf,
	Final,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stmt {
	pub span: Span,
	pub label: Option<Name>,
	pub data: StmtData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StmtData {
	NullStmt,
	SequentialBlock(Vec<Stmt>),
	ParallelBlock(Vec<Stmt>, JoinKind),
	IfStmt {
		up: Option<UniquePriority>,
		cond: Expr,
		main_stmt: Box<Stmt>,
		else_stmt: Option<Box<Stmt>>
	},
	BlockingAssignStmt {
		lhs: Expr,
		rhs: Expr,
		op: AssignOp,
	},
	NonblockingAssignStmt {
		lhs: Expr,
		rhs: Expr,
		delay: Option<DelayControl>,
		event: Option<()>,
	},
	TimedStmt(TimingControl, Box<Stmt>),
	CaseStmt {
		up: Option<UniquePriority>,
		kind: CaseKind,
		expr: Expr,
		mode: CaseMode,
		items: Vec<CaseItem>,
	},
	ForeverStmt(Box<Stmt>),
	RepeatStmt(Expr, Box<Stmt>),
	WhileStmt(Expr, Box<Stmt>),
	DoStmt(Box<Stmt>, Expr),
	ForStmt(Box<Stmt>, Expr, Expr, Box<Stmt>),
	ForeachStmt(Expr, Box<Stmt>),
	ExprStmt(Expr),
	VarDeclStmt(VarDecl),
	GenvarDeclStmt(Vec<GenvarDecl>),
	ContinueStmt,
	BreakStmt,
	ReturnStmt(Option<Expr>),
	ImportStmt(ImportDecl),
	AssertionStmt(Box<Assertion>),
}

impl Stmt {
	pub fn new_null(span: Span) -> Stmt {
		Stmt {
			span: span,
			label: None,
			data: NullStmt,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinKind {
	All,
	Any,
	None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UniquePriority {
	Unique,
	Unique0,
	Priority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseKind {
	Normal,
	DontCareZ,
	DontCareXZ,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseMode {
	Normal,
	Inside,
	Pattern,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaseItem {
	Default(Box<Stmt>),
	Expr(Vec<Expr>, Box<Stmt>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelayControl {
	pub span: Span,
	pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventControl {
	pub span: Span,
	pub data: EventControlData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventControlData {
	Implicit,
	Expr(EventExpr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleDelay {

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimingControl {
	Delay(DelayControl),
	Event(EventControl),
	Cycle(CycleDelay),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignOp {
	Identity,
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	BitAnd,
	BitOr,
	BitXor,
	LogicShL,
	LogicShR,
	ArithShL,
	ArithShR,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarDecl {
	pub span: Span,
	pub ty: Type,
	pub names: Vec<VarDeclName>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarDeclName {
	pub span: Span,
	pub name: Name,
	pub name_span: Span,
	pub dims: Vec<TypeDim>,
	pub init: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenvarDecl {
	pub span: Span,
	pub name: Name,
	pub name_span: Span,
	pub init: Option<Expr>,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
	pub span: Span,
	pub data: ExprData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprData {
	DummyExpr,
	CallExpr(Box<Expr>, Vec<CallArg>),
	TypeExpr(Box<Type>),
	ConstructorCallExpr(Vec<CallArg>),
	ClassNewExpr(Option<Box<Expr>>),
	ArrayNewExpr(Box<Expr>, Option<Box<Expr>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallArg {
	pub span: Span,
	pub name_span: Span,
	pub name: Option<Name>,
	pub expr: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamConcatSlice {
	Expr(Box<Expr>),
	Type(Type),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamExpr {
	pub expr: Box<Expr>,
	pub range: Option<Box<Expr>>,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventExpr {
	Edge {
		span:  Span,
		edge:  EdgeIdent,
		value: Expr,
	},
	Iff {
		span: Span,
		expr: Box<EventExpr>,
		cond: Expr,
	},
	Or {
		span: Span,
		lhs: Box<EventExpr>,
		rhs: Box<EventExpr>,
	},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeIdent {
	Implicit,
	Edge,
	Posedge,
	Negedge,
}

impl EventExpr {
	pub fn span(&self) -> Span {
		match *self {
			EventExpr::Edge { span: sp, .. } => sp,
			EventExpr::Iff { span: sp, .. } => sp,
			EventExpr::Or { span: sp, .. } => sp,
		}
	}
}



#[derive(Debug)]
pub struct ClassDecl {
	pub span: Span,
	pub virt: bool,
	pub lifetime: Lifetime, // default static
	pub name: Name,
	pub name_span: Span,
	pub params: Vec<()>,
	pub extends: Option<(Type, Vec<CallArg>)>,
	pub items: Vec<ClassItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassItem {
	pub span: Span,
	pub qualifiers: Vec<(ClassItemQualifier,Span)>,
	pub data: ClassItemData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassItemQualifier {
	Static,
	Protected,
	Local,
	Rand,
	Randc,
	Pure,
	Virtual,
	Const,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassItemData {
	Property,
	SubroutineDecl(SubroutineDecl),
	ExternSubroutine(SubroutinePrototype),
	Constraint(Constraint),
	ClassDecl,
	CovergroupDecl,
	LocalParamDecl(()),
	ParameterDecl(()),
	Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomQualifier {
	Rand,
	Randc,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Typedef {
	pub span: Span,
	pub name: Name,
	pub name_span: Span,
	pub ty: Type,
	pub dims: Vec<TypeDim>,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
	pub span: Span,
	pub kind: ConstraintKind,
	pub statik: bool,
	pub name: Name,
	pub name_span: Span,
	pub items: Vec<ConstraintItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintKind {
	Decl,
	Proto,
	ExternProto,
	PureProto,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintItem {
	pub span: Span,
	pub data: ConstraintItemData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintItemData {
	If,
	Foreach,
	Expr(Expr),
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubroutineDecl {
	pub span: Span,
	pub prototype: SubroutinePrototype,
	pub items: Vec<SubroutineItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubroutinePrototype {
	pub span: Span,
	pub kind: SubroutineKind,
	pub name: Name,
	pub name_span: Span,
	pub args: Vec<SubroutinePort>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SubroutineKind {
	Func,
	Task,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubroutinePort {
	pub span: Span,
	pub dir: Option<SubroutinePortDir>,
	pub var: bool,
	pub ty: Type,
	pub name: Option<SubroutinePortName>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubroutinePortName {
	pub name: Name,
	pub name_span: Span,
	pub dims: Vec<TypeDim>,
	pub expr: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubroutineItem {
	PortDecl(SubroutinePortDecl),
	Stmt(Stmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubroutinePortDecl {
	pub span: Span,
	pub dir: SubroutinePortDir,
	pub var: bool,
	pub ty: Type,
	pub names: Vec<VarDeclName>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubroutinePortDir {
	Input,
	Output,
	Inout,
	Ref,
	ConstRef,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetDecl {
	pub span: Span,
	pub net_type: NetType,
	pub strength: Option<NetStrength>,
	pub kind: NetKind,
	pub ty: Type,
	pub delay: Option<Expr>,
	pub names: Vec<VarDeclName>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetKind {
	Vectored,
	Scalared,
	None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetStrength {
	Drive(DriveStrength, DriveStrength),
	Charge(ChargeStrength),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriveStrength {
	Supply0,
	Strong0,
	Pull0,
	Weak0,
	HighZ0,
	Supply1,
	Strong1,
	Pull1,
	Weak1,
	HighZ1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChargeStrength {
	Small,
	Medium,
	Large,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternField {
	pub span: Span,
	pub data: PatternFieldData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternFieldData {
	Default(Box<Expr>),
	Member(Box<Expr>, Box<Expr>),
	Type(Type, Box<Expr>),
	Expr(Box<Expr>),
	Repeat(Box<Expr>, Vec<Expr>),
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportDecl {
	pub span: Span,
	pub items: Vec<ImportItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportItem {
	pub pkg: Name,
	pub pkg_span: Span,
	pub name: Option<Name>, // None means `import pkg::*`
	pub name_span: Span,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assertion {
	pub span: Span,
	pub label: Option<(Name, Span)>,
	pub data: AssertionData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssertionData {
	Immediate(BlockingAssertion),
	Deferred(BlockingAssertion),
	Concurrent(ConcurrentAssertion),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockingAssertion {
	Assert(Expr, AssertionActionBlock),
	Assume(Expr, AssertionActionBlock),
	Cover(Expr, Stmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcurrentAssertion {
	AssertProperty(PropSpec, AssertionActionBlock),
	AssumeProperty(PropSpec, AssertionActionBlock),
	CoverProperty(PropSpec, Stmt),
	CoverSequence,
	ExpectProperty(PropSpec, AssertionActionBlock),
	RestrictProperty(PropSpec),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssertionActionBlock {
	Positive(Stmt),
	Negative(Stmt),
	Both(Stmt, Stmt),
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeqExpr {
	pub span: Span,
	pub data: SeqExprData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeqExprData {
	Expr(Expr, Option<SeqRep>),
	BinOp(SeqBinOp, Box<SeqExpr>, Box<SeqExpr>),
	Throughout(Expr, Box<SeqExpr>),
	Clocked(EventExpr, Box<SeqExpr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeqRep {
	Consec(Expr),    // [* expr]
	ConsecStar,      // [*]
	ConsecPlus,      // [+]
	Nonconsec(Expr), // [= expr]
	Goto(Expr),      // [-> expr]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeqBinOp {
	Or,
	And,
	Intersect,
	Within,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropSpec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropExpr {
	pub span: Span,
	pub data: PropExprData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropExprData {
	SeqOp(PropSeqOp, SeqExpr),
	SeqBinOp(PropSeqBinOp, PropSeqOp, SeqExpr, Box<PropExpr>),
	Not(Box<PropExpr>),
	BinOp(PropBinOp, Box<PropExpr>, Box<PropExpr>),
	Clocked(EventExpr, Box<PropExpr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropSeqOp {
	None,
	Weak,
	Strong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropSeqBinOp {
	ImplOverlap,
	ImplNonoverlap,
	FollowOverlap,
	FollowNonoverlap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropBinOp {
	Or,
	And,
	Until,
	SUntil,
	UntilWith,
	SUntilWith,
	Impl,
	Iff,
	SeqImplOl,
	SeqImplNol,
	SeqFollowOl,
	SeqFollowNol,
}
