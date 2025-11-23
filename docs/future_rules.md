# Future Rules

Potential rules to implement based on examples from https://github.com/JelleZijlstra/unsoundness

## TypeGuard

### typing-typeguard-used
Flag `TypeGuard` in return types. If you write a bad TypeGuard function that always returns True, type narrowing still happens even though the type is wrong at runtime.

## Context Managers

### unsafe-exception-suppression
Context managers that swallow exceptions are a problem. If a context manager catches and suppresses an exception, code inside the block might not execute fully - so variables end up with the wrong type.

### unsafe-context-manager-binding
Exceptions during `__enter__` can mess with control flow and break type contracts.

## Descriptors

### descriptor-instance-dict-access
When you put a descriptor in an instance's `__dict__`, the descriptor protocol gets bypassed - `__get__`/`__set__` won't run.

### descriptor-protocol-override
Subclasses adding `__get__`, `__set__`, or `__delete__` change how attribute access works, but type checkers can't see this.

## Directives

### unsafe-override-used
The `@unsafe_override` decorator literally tells the type checker to allow unsafe overrides.

## Metaclasses

### metaclass-new-override
Custom `__new__` in metaclasses can change how classes get created, breaking type annotations.

## Narrowing

### attribute-narrowing-used
Type narrowing based on whether an attribute exists isn't always safe - no guarantee it matches runtime behavior.

### attribute-assignment-narrowing-used
Similar issue with narrowing from attribute assignments.

### isinstance-dict-used
`isinstance(x, dict)` doesn't properly narrow TypedDict types.

### isinstance-list-used
`isinstance(x, list)` checks lose generic type parameters like `list[int]`.

### match-mapping-narrowing
Pattern matching on mappings can narrow types wrong.

### narrowing-descriptor-attribute
Narrowing descriptor attributes doesn't account for how descriptors actually work.

## Overload

### overlapping-overload-signatures
When overloads overlap, the type checker picks one but runtime might use another, so you get the wrong return type.

## Override

### override-descriptor-get
Child classes overriding `__get__` changes descriptor behavior without type checkers noticing.

### dataclass-replace-override
`dataclasses.replace()` doesn't handle overridden fields properly.

### override-init-signature
Child `__init__` with different parameter types violates LSP.

## Protocols

### protocol-self-reference
Protocols that reference themselves have variance problems.

### runtime-checkable-protocol-used
`@runtime_checkable` only checks if methods exist, not their actual signatures. Wrong types slip through.

## Runtime Modification

### exec-used
`exec()` can redefine functions/classes at runtime, making type annotations lies.

## Stdlib

### dict-constructor-unsound
Dict constructor loses TypedDict type info.

### mutable-inplace-operation
In-place ops like `+=` on lists don't preserve generic types.

### pow-any-usage
`pow()` with `Any` has weird type semantics that go unsound.

## Tuple

### tuple-subclass-constructor
Tuple subclass constructors have variance issues.

### tuple-iteration-unsound
When you iterate a tuple, you lose the per-element types and get a union instead.

## TypedDict

### typeddict-update-used
`.update()` on TypedDict can replace fields with incompatible types when using base class types.

### typeddict-argument-unpacking
Unpacking TypedDict into function args doesn't preserve required vs optional.

### typeddict-or-equals-used
`|=` on TypedDict merges incompatible types.

## Miscellaneous

### method-instance-type-confusion
You can call class methods with instances of the wrong subclass type and the type checker doesn't catch it.
