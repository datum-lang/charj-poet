///
/// A fragment of a .kt file, potentially containing declarations, statements, and documentation.
/// Code blocks are not necessarily well-formed Kotlin code, and are not validated. This class
/// assumes kotlinc will check correctness later!
///
/// Code blocks support placeholders like [java.text.Format]. This class primarily uses a percent
/// sign `%` but has its own set of permitted placeholders:
///
/// * `%L` emits a *literal* value with no escaping. Arguments for literals may be strings,
///   primitives, [type declarations][TypeSpec], [annotations][AnnotationSpec] and even other code
///   blocks.
/// * `%N` emits a *name*, using name collision avoidance where necessary. Arguments for names may
///   be strings (actually any [character sequence][CharSequence]), [parameters][ParameterSpec],
///   [properties][PropertySpec], [functions][FunSpec], and [types][TypeSpec].
/// * `%S` escapes the value as a *string*, wraps it with double quotes, and emits that. For
///   example, `6" sandwich` is emitted `"6\" sandwich"`. `%S` will also escape all dollar signs
///   (`$`), use `%P` for string templates.
/// * `%P` - Similar to `%S`, but doesn't escape dollar signs (`$`) to allow creation of string
///   templates. If the string contains dollar signs that should be escaped - use `%S`.
/// * `%T` emits a *type* reference. Types will be imported if possible. Arguments for types may be
///   [classes][Class].
/// * `%M` emits a *member* reference. A member is either a function or a property. If the member is
///   importable, e.g. it's a top-level function or a property declared inside an object, the import
///   will be resolved if possible. Arguments for members must be of type [MemberName].
/// * `%%` emits a percent sign.
/// * `·` emits a space that never wraps. KotlinPoet prefers to wrap lines longer than 100 columns.
///   It does this by replacing normal spaces with a newline and indent. Note that spaces in strings
///   are never wrapped.
/// * `⇥` increases the indentation level.
/// * `⇤` decreases the indentation level.
/// * `«` begins a statement. For multiline statements, every line after the first line is
///   double-indented.
/// * `»` ends a statement.
///

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CodeBlock {

}
