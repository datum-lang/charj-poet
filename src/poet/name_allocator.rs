/**
 * Assigns Java identifier names to avoid collisions, keywords, and invalid characters. To use,
 * first create an instance and allocate all of the names that you need. Typically this is a
 * mix of user-supplied names and constants: <pre>   {@code
 *
 *   NameAllocator nameAllocator = new NameAllocator();
 *   for (MyProperty property : properties) {
 *     nameAllocator.newName(property.name(), property);
 *   }
 *   nameAllocator.newName("sb", "string builder");
 * }</pre>
 *
 * Pass a unique tag object to each allocation. The tag scopes the name, and can be used to look up
 * the allocated name later. Typically the tag is the object that is being named. In the above
 * example we use {@code property} for the user-supplied property names, and {@code "string
 * builder"} for our constant string builder.
 *
 * <p>Once we've allocated names we can use them when generating code: <pre>   {@code
 *
 *   MethodSpec.Builder builder = MethodSpec.methodBuilder("toString")
 *       .addAnnotation(Override.class)
 *       .addModifiers(Modifier.PUBLIC)
 *       .returns(String.class);
 *
 *   builder.addStatement("$1T $2N = new $1T()",
 *       StringBuilder.class, nameAllocator.get("string builder"));
 *   for (MyProperty property : properties) {
 *     builder.addStatement("$N.append($N)",
 *         nameAllocator.get("string builder"), nameAllocator.get(property));
 *   }
 *   builder.addStatement("return $N", nameAllocator.get("string builder"));
 *   return builder.build();
 * }</pre>
 *
 * The above code generates unique names if presented with conflicts. Given user-supplied properties
 * with names {@code ab} and {@code sb} this generates the following:  <pre>   {@code
 *
 *   &#64;Override
 *   public String toString() {
 *     StringBuilder sb_ = new StringBuilder();
 *     sb_.append(ab);
 *     sb_.append(sb);
 *     return sb_.toString();
 *   }
 * }</pre>
 *
 * The underscore is appended to {@code sb} to avoid conflicting with the user-supplied {@code sb}
 * property. Underscores are also prefixed for names that start with a digit, and used to replace
 * name-unsafe characters like space or dash.
 *
 * <p>When dealing with multiple independent inner scopes, use a {@link #clone()} of the
 * NameAllocator used for the outer scope to further refine name allocation for a specific inner
 * scope.
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NameAllocator {}
