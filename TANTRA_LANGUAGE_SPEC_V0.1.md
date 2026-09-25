# TANTRA LANGUAGE SPECIFICATION v0.1

**Status:** Designed baseline / pre-implementation specification  
**Stage:** 001 — Formal Language Specification  
**Source extension:** `.tantra`

## 1. Purpose
This document converts the existing master blueprint into an implementation-oriented core-language contract. It is a specification, not evidence that the language or compiler is implemented.

## 2. Core principles
- Deterministic parsing and formatter-friendly syntax.
- Strong static typing and explicit mutability.
- Explicit Option/Result handling.
- Explicit security capabilities/effects.
- Memory/resource safety by default.
- UTF-8 source with restricted, normalized Unicode identifiers.
- No unrestricted dynamic code execution.
- No implicit dangerous or lossy conversions.
- Safe core first; unsafe/FFI behavior requires a separate security specification.

## 3. Source and lexical rules
- Source encoding: UTF-8.
- Identifier normalization: NFC. The lexer canonicalizes each identifier token to NFC while preserving the original byte span.
- Identifiers are case-sensitive.
- Control characters are forbidden in identifiers.
- A conservative UTS #39 confusable policy is enforced in the lexer: a non-ASCII identifier character whose confusable prototype is a single ASCII alphanumeric is rejected.
- Unicode confusable detection remains a broader tooling/security area beyond this lexical baseline.
- Line comments use //.
- Block comments use /* ... */ and may nest.
- Whitespace is insignificant except inside literals/comments.

## 4. Canonical v0.1 keywords
સ્થિર, બદલ, કાર્ય, પરત, જો, નહીં, તો, માટે, દરેક, માં, જ્યારે, તોડો, આગળ, આયાત,માંથી, રૂપ, પ્રયત્ન, ભૂલ, ફેંકો, સાચું, ખોટું, શૂન્ય, async, await, ક્ષમતા, જાહેર, ખાનગી, મોડ્યુલ.

## 5. Literals
- Boolean: સાચું / ખોટું.
- Absence: શૂન્ય; ordinary non-Option types cannot contain it.
- Integer: decimal, 0x hexadecimal, 0b binary, 0o octal.
- Floating point: decimal/exponent notation.
- Character: single quotes.
- String: double quotes.
- Supported string escapes: \\n, \\r, \\t, \\", \\\\, \\0.
- Supported character escapes: \\n, \\r, \\t, \\', \\\\, \\0.
- Numeric separators using underscore are allowed only between digits; leading, trailing and repeated underscores are invalid.
- Prefixed integer literals must contain at least one valid base digit and cannot contain digits/letters outside the selected radix.
- Decimal exponent notation must contain at least one digit after the optional sign.
- Malformed numeric literals are rejected with stable diagnostic code T0009.

## 6. Operators and precedence
Highest to lowest:
1. member/index/call
2. unary ! + -
3. exponent **
4. * / %
5. + -
6. << >>
7. < <= > >=
8. == !=
9. &
10. ^
11. |
12. &&
13. ||
14. ??
15. ?:
16. assignment = += -= *= /= %=
Assignment is right-associative; exponentiation is right-associative; other binary operators are left-associative.

## 7. Variables
Immutable:
સ્થિર સંખ્યા: પૂર્ણાંક = 10
Mutable:
બદલ ગણતરી: પૂર્ણાંક = 0
Type inference is permitted when unambiguous. Immutable bindings cannot be reassigned. Use-before-initialization is an error.

## 8. Functions
Example:
કાર્ય ઉમેરો(a: પૂર્ણાંક, b: પૂર્ણાંક) -> પૂર્ણાંક {
    પરત a + b
}
Functions are first-class values. Recursion is allowed. Public functions require explicit return types in v0.1. Overload resolution is deferred.

## 9. Control flow
Conditional form:
જો condition { ... } نہیں જો condition { ... } નહીં { ... }
Loop forms:
જ્યારે condition { ... }
માટે દરેક item માં items { ... }
Loop control: તોડો and આગળ. Conditions must be Boolean.

## 10. Modules
Files are modules by default. Imports use આયાત. Declarations are private by default; જાહેર makes a declaration externally visible. Circular module dependencies are rejected in v0.1.

## 11. Type system
Primitive families: Boolean, signed/unsigned integers, Float/Decimal families, Char, String and Byte.
Compound types: Array<T>, Map<K,V>, Set<T>, Tuple, Option<T>, Result<T,E>, user-defined struct/enum types and function types.
Generic syntax uses Type<A,B>.
User-defined type example:
રૂપ વ્યક્તિ {
    નામ: શબ્દ
    ઉમર: પૂર્ણાંક
}
No implicit dangerous casts are allowed.

## 12. Nullability
Option<T> represents absence. A plain T cannot contain શૂન્ય. Unwrapping must be explicit and compiler-checked. ?? provides null-coalescing behavior.

## 13. Error model
Recoverable failures use Result<T,E>. Structured exceptional handling uses પ્રયત્ન { ... } ભૂલ err { ... }. Expected failures should not require untyped exceptions.

## 14. Async/concurrency
async functions may use await. await is legal only in async context. The safe model requires structured task ownership and explicit synchronization for shared mutable state. Exact scheduler behavior is runtime-specific.

## 15. Memory/resource safety
Safe code must not permit arbitrary memory access, use-after-free, double-free, or unchecked collection access. Resources such as files and sockets require explicit ownership/cleanup semantics.
An unsafe keyword and arbitrary native FFI are deferred until a dedicated security/FFI RFC.

## 16. Capability/effect system
Privileged functions declare required capabilities, for example:
કાર્ય download(url: શબ્દ) -> Result<બાઇટ[], નેટવર્કભૂલ>
    ક્ષમતા network.read
{ ... }
Initial namespaces: network.read, network.write, filesystem.read, filesystem.write, process.spawn, crypto.use, camera.read, microphone.read, location.read, clipboard.read, clipboard.write, secrets.read.
Importing an API does not grant its capability. Capability requirements propagate through calls and cannot be silently escalated.

## 17. Sensitive types
Future standard-library/core security APIs include Secret<T>, AccessToken, ApiKey and PrivateKey. Secret values must be redacted from ordinary diagnostics/log formatting.

## 18. Conversion and numeric safety
Only provably safe literal/inference conversions may be implicit. Other conversions are explicit. Integer overflow behavior must be specified by the operation and may be checked, wrapping, saturating or trapping; undefined signed overflow is forbidden.

## 19. Deferred language features
Traits/interfaces, advanced pattern matching, operator overloading, macros, reflection, dynamic Any-like types, arbitrary FFI, unsafe syntax, package-registry protocol, UI DSL, graphics DSL, advanced tensor syntax, native ABI and formal-verification syntax are deferred.

## 20. Diagnostics contract
Every compiler diagnostic should expose a stable code, severity, source span, message, expected/actual information where useful, related spans and safe suggestions.
Example code: T1003 — Type mismatch.
Diagnostics are part of the language/tooling API and must not leak secrets unnecessarily.

## 21. EBNF baseline
program = { declaration | statement } ;
declaration = variable_decl | function_decl | type_decl | import_decl | module_decl ;
variable_decl = (સ્થિર | બદલ) identifier [ : type ] = expression ;
function_decl = [જાહેર] [async] કાર્ય identifier ( [parameters] ) [ -> type ] [capability_clause] block ;
parameters = parameter { , parameter } ;
parameter = identifier : type ;
capability_clause = { ક્ષમતા capability_name } ;
type_decl = રૂપ identifier [generic_params] struct_type | રૂપ identifier [generic_params] enum_type ;
statement = block | if_stmt | while_stmt | for_stmt | return_stmt | break_stmt | continue_stmt | try_stmt | expression_stmt ;
block = { { declaration | statement } } ;
if_stmt = જો expression block { નહીં જો expression block } [નહીં block] ;
while_stmt = જ્યારે expression block ;
for_stmt = માટે દરેક identifier માં expression block ;
return_stmt = પરત [expression] ;
break_stmt = તોડો ;
continue_stmt = આગળ ;
expression_stmt = expression ;
expression = assignment ;
assignment = conditional [assignment_operator assignment] ;
conditional = null_coalesce [ ? expression : expression ] ;
null_coalesce = logical_or { ?? logical_or } ;
logical_or = logical_and { || logical_and } ;
logical_and = equality { && equality } ;
equality = comparison { (== | !=) comparison } ;
comparison = additive { (< | <= | > | >=) additive } ;
additive = multiplicative { (+ | -) multiplicative } ;
multiplicative = exponent { (* | / | %) exponent } ;
exponent = unary [ ** exponent ] ;
unary = [! | + | -] postfix ;
postfix = primary { call | index | member } ;
primary = literal | identifier | ( expression ) | array_literal ;

Note: lexical token definitions, exact Unicode identifier grammar and the complete operator grammar will be encoded in the executable parser tests before implementation is considered conforming.

## 22. AST requirements
AST nodes must preserve source spans, declaration order, mutability, capability declarations, async markers and type annotations required for diagnostics and later passes.

## 23. Name resolution
The resolver creates module/lexical scopes, resolves imports and identifiers, detects illegal duplicates and records symbol/source relationships. No unresolved name may reach typed IR.

## 24. Type checking
The checker verifies assignment compatibility, operators, calls, returns, Boolean conditions, collections, generics, Option/Result rules, capability requirements and async/await context. Ambiguity fails closed.

## 25. Security invariants
1. Safe code has no arbitrary memory access.
2. Safe collection access is bounds-checked.
3. Privileges are not acquired merely by importing APIs.
4. Secrets are not emitted in ordinary diagnostics.
5. Dynamic source execution is not part of the safe core.
6. Lossy conversions are explicit.
7. Unresolved code cannot execute.
8. Unsafe/native functionality stays outside the safe core until specified.

## 26. Conformance
A v0.1 implementation is conforming only after passing executable tests for lexical parsing, valid/invalid syntax, name resolution, typing, capabilities, nullability, control flow, diagnostics and security-negative cases.

## 27. Status
Specification version: 0.1.
Current status: Specified. Not implemented.
Any later breaking change must update this document, affected tests, documentation and handover.

## 28. Implementation order
1. UTF-8 source reader.
2. Token model and lexer.
3. Diagnostics.
4. Parser and AST.
5. Name resolver.
6. Primitive type checker.
7. Functions/types/control flow.
8. Option/Result.
9. Capability checker.
10. Typed IR.
11. Tiny evaluator/conformance runner.
12. Initial backend.

## 29. Stage record
Created during Stage 001 after repository-wide inspection and reading the project-control files. The master blueprint remains the higher-level architectural source of truth; this document is the current core-language specification baseline.