// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/clone_in.rs`

#![allow(clippy::default_trait_access)]

use oxc_allocator::{Allocator, CloneIn};

use crate::ast::*;

impl<'new_alloc> CloneIn<'new_alloc> for Pattern<'_> {
    type Cloned = Pattern<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        Pattern {
            span: CloneIn::clone_in(&self.span, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        Pattern {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            body: CloneIn::clone_in_with_semantic_ids(&self.body, allocator),
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for Disjunction<'_> {
    type Cloned = Disjunction<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        Disjunction {
            span: CloneIn::clone_in(&self.span, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        Disjunction {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            body: CloneIn::clone_in_with_semantic_ids(&self.body, allocator),
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for Alternative<'_> {
    type Cloned = Alternative<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        Alternative {
            span: CloneIn::clone_in(&self.span, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        Alternative {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            body: CloneIn::clone_in_with_semantic_ids(&self.body, allocator),
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for Term<'_> {
    type Cloned = Term<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        match self {
            Self::BoundaryAssertion(it) => {
                Term::BoundaryAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::LookAroundAssertion(it) => {
                Term::LookAroundAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::Quantifier(it) => Term::Quantifier(CloneIn::clone_in(it, allocator)),
            Self::Character(it) => Term::Character(CloneIn::clone_in(it, allocator)),
            Self::Dot(it) => Term::Dot(CloneIn::clone_in(it, allocator)),
            Self::CharacterClassEscape(it) => {
                Term::CharacterClassEscape(CloneIn::clone_in(it, allocator))
            }
            Self::UnicodePropertyEscape(it) => {
                Term::UnicodePropertyEscape(CloneIn::clone_in(it, allocator))
            }
            Self::CharacterClass(it) => Term::CharacterClass(CloneIn::clone_in(it, allocator)),
            Self::CapturingGroup(it) => Term::CapturingGroup(CloneIn::clone_in(it, allocator)),
            Self::IgnoreGroup(it) => Term::IgnoreGroup(CloneIn::clone_in(it, allocator)),
            Self::IndexedReference(it) => Term::IndexedReference(CloneIn::clone_in(it, allocator)),
            Self::NamedReference(it) => Term::NamedReference(CloneIn::clone_in(it, allocator)),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        match self {
            Self::BoundaryAssertion(it) => {
                Term::BoundaryAssertion(CloneIn::clone_in_with_semantic_ids(it, allocator))
            }
            Self::LookAroundAssertion(it) => {
                Term::LookAroundAssertion(CloneIn::clone_in_with_semantic_ids(it, allocator))
            }
            Self::Quantifier(it) => {
                Term::Quantifier(CloneIn::clone_in_with_semantic_ids(it, allocator))
            }
            Self::Character(it) => {
                Term::Character(CloneIn::clone_in_with_semantic_ids(it, allocator))
            }
            Self::Dot(it) => Term::Dot(CloneIn::clone_in_with_semantic_ids(it, allocator)),
            Self::CharacterClassEscape(it) => {
                Term::CharacterClassEscape(CloneIn::clone_in_with_semantic_ids(it, allocator))
            }
            Self::UnicodePropertyEscape(it) => {
                Term::UnicodePropertyEscape(CloneIn::clone_in_with_semantic_ids(it, allocator))
            }
            Self::CharacterClass(it) => {
                Term::CharacterClass(CloneIn::clone_in_with_semantic_ids(it, allocator))
            }
            Self::CapturingGroup(it) => {
                Term::CapturingGroup(CloneIn::clone_in_with_semantic_ids(it, allocator))
            }
            Self::IgnoreGroup(it) => {
                Term::IgnoreGroup(CloneIn::clone_in_with_semantic_ids(it, allocator))
            }
            Self::IndexedReference(it) => {
                Term::IndexedReference(CloneIn::clone_in_with_semantic_ids(it, allocator))
            }
            Self::NamedReference(it) => {
                Term::NamedReference(CloneIn::clone_in_with_semantic_ids(it, allocator))
            }
        }
    }
}

impl<'alloc> CloneIn<'alloc> for BoundaryAssertion {
    type Cloned = BoundaryAssertion;
    fn clone_in(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        BoundaryAssertion {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        BoundaryAssertion {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            kind: CloneIn::clone_in_with_semantic_ids(&self.kind, allocator),
        }
    }
}

impl<'alloc> CloneIn<'alloc> for BoundaryAssertionKind {
    type Cloned = BoundaryAssertionKind;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Start => BoundaryAssertionKind::Start,
            Self::End => BoundaryAssertionKind::End,
            Self::Boundary => BoundaryAssertionKind::Boundary,
            Self::NegativeBoundary => BoundaryAssertionKind::NegativeBoundary,
        }
    }

    fn clone_in_with_semantic_ids(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Start => BoundaryAssertionKind::Start,
            Self::End => BoundaryAssertionKind::End,
            Self::Boundary => BoundaryAssertionKind::Boundary,
            Self::NegativeBoundary => BoundaryAssertionKind::NegativeBoundary,
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for LookAroundAssertion<'_> {
    type Cloned = LookAroundAssertion<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        LookAroundAssertion {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        LookAroundAssertion {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            kind: CloneIn::clone_in_with_semantic_ids(&self.kind, allocator),
            body: CloneIn::clone_in_with_semantic_ids(&self.body, allocator),
        }
    }
}

impl<'alloc> CloneIn<'alloc> for LookAroundAssertionKind {
    type Cloned = LookAroundAssertionKind;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Lookahead => LookAroundAssertionKind::Lookahead,
            Self::NegativeLookahead => LookAroundAssertionKind::NegativeLookahead,
            Self::Lookbehind => LookAroundAssertionKind::Lookbehind,
            Self::NegativeLookbehind => LookAroundAssertionKind::NegativeLookbehind,
        }
    }

    fn clone_in_with_semantic_ids(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Lookahead => LookAroundAssertionKind::Lookahead,
            Self::NegativeLookahead => LookAroundAssertionKind::NegativeLookahead,
            Self::Lookbehind => LookAroundAssertionKind::Lookbehind,
            Self::NegativeLookbehind => LookAroundAssertionKind::NegativeLookbehind,
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for Quantifier<'_> {
    type Cloned = Quantifier<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        Quantifier {
            span: CloneIn::clone_in(&self.span, allocator),
            min: CloneIn::clone_in(&self.min, allocator),
            max: CloneIn::clone_in(&self.max, allocator),
            greedy: CloneIn::clone_in(&self.greedy, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        Quantifier {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            min: CloneIn::clone_in_with_semantic_ids(&self.min, allocator),
            max: CloneIn::clone_in_with_semantic_ids(&self.max, allocator),
            greedy: CloneIn::clone_in_with_semantic_ids(&self.greedy, allocator),
            body: CloneIn::clone_in_with_semantic_ids(&self.body, allocator),
        }
    }
}

impl<'alloc> CloneIn<'alloc> for Character {
    type Cloned = Character;
    fn clone_in(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        Character {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        Character {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            kind: CloneIn::clone_in_with_semantic_ids(&self.kind, allocator),
            value: CloneIn::clone_in_with_semantic_ids(&self.value, allocator),
        }
    }
}

impl<'alloc> CloneIn<'alloc> for CharacterKind {
    type Cloned = CharacterKind;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::ControlLetter => CharacterKind::ControlLetter,
            Self::HexadecimalEscape => CharacterKind::HexadecimalEscape,
            Self::Identifier => CharacterKind::Identifier,
            Self::Null => CharacterKind::Null,
            Self::Octal1 => CharacterKind::Octal1,
            Self::Octal2 => CharacterKind::Octal2,
            Self::Octal3 => CharacterKind::Octal3,
            Self::SingleEscape => CharacterKind::SingleEscape,
            Self::Symbol => CharacterKind::Symbol,
            Self::UnicodeEscape => CharacterKind::UnicodeEscape,
        }
    }

    fn clone_in_with_semantic_ids(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::ControlLetter => CharacterKind::ControlLetter,
            Self::HexadecimalEscape => CharacterKind::HexadecimalEscape,
            Self::Identifier => CharacterKind::Identifier,
            Self::Null => CharacterKind::Null,
            Self::Octal1 => CharacterKind::Octal1,
            Self::Octal2 => CharacterKind::Octal2,
            Self::Octal3 => CharacterKind::Octal3,
            Self::SingleEscape => CharacterKind::SingleEscape,
            Self::Symbol => CharacterKind::Symbol,
            Self::UnicodeEscape => CharacterKind::UnicodeEscape,
        }
    }
}

impl<'alloc> CloneIn<'alloc> for CharacterClassEscape {
    type Cloned = CharacterClassEscape;
    fn clone_in(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        CharacterClassEscape {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        CharacterClassEscape {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            kind: CloneIn::clone_in_with_semantic_ids(&self.kind, allocator),
        }
    }
}

impl<'alloc> CloneIn<'alloc> for CharacterClassEscapeKind {
    type Cloned = CharacterClassEscapeKind;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::D => CharacterClassEscapeKind::D,
            Self::NegativeD => CharacterClassEscapeKind::NegativeD,
            Self::S => CharacterClassEscapeKind::S,
            Self::NegativeS => CharacterClassEscapeKind::NegativeS,
            Self::W => CharacterClassEscapeKind::W,
            Self::NegativeW => CharacterClassEscapeKind::NegativeW,
        }
    }

    fn clone_in_with_semantic_ids(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::D => CharacterClassEscapeKind::D,
            Self::NegativeD => CharacterClassEscapeKind::NegativeD,
            Self::S => CharacterClassEscapeKind::S,
            Self::NegativeS => CharacterClassEscapeKind::NegativeS,
            Self::W => CharacterClassEscapeKind::W,
            Self::NegativeW => CharacterClassEscapeKind::NegativeW,
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for UnicodePropertyEscape<'_> {
    type Cloned = UnicodePropertyEscape<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        UnicodePropertyEscape {
            span: CloneIn::clone_in(&self.span, allocator),
            negative: CloneIn::clone_in(&self.negative, allocator),
            strings: CloneIn::clone_in(&self.strings, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        UnicodePropertyEscape {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            negative: CloneIn::clone_in_with_semantic_ids(&self.negative, allocator),
            strings: CloneIn::clone_in_with_semantic_ids(&self.strings, allocator),
            name: CloneIn::clone_in_with_semantic_ids(&self.name, allocator),
            value: CloneIn::clone_in_with_semantic_ids(&self.value, allocator),
        }
    }
}

impl<'alloc> CloneIn<'alloc> for Dot {
    type Cloned = Dot;
    fn clone_in(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        Dot { span: CloneIn::clone_in(&self.span, allocator) }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        Dot { span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator) }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for CharacterClass<'_> {
    type Cloned = CharacterClass<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        CharacterClass {
            span: CloneIn::clone_in(&self.span, allocator),
            negative: CloneIn::clone_in(&self.negative, allocator),
            strings: CloneIn::clone_in(&self.strings, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        CharacterClass {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            negative: CloneIn::clone_in_with_semantic_ids(&self.negative, allocator),
            strings: CloneIn::clone_in_with_semantic_ids(&self.strings, allocator),
            kind: CloneIn::clone_in_with_semantic_ids(&self.kind, allocator),
            body: CloneIn::clone_in_with_semantic_ids(&self.body, allocator),
        }
    }
}

impl<'alloc> CloneIn<'alloc> for CharacterClassContentsKind {
    type Cloned = CharacterClassContentsKind;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Union => CharacterClassContentsKind::Union,
            Self::Intersection => CharacterClassContentsKind::Intersection,
            Self::Subtraction => CharacterClassContentsKind::Subtraction,
        }
    }

    fn clone_in_with_semantic_ids(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Union => CharacterClassContentsKind::Union,
            Self::Intersection => CharacterClassContentsKind::Intersection,
            Self::Subtraction => CharacterClassContentsKind::Subtraction,
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for CharacterClassContents<'_> {
    type Cloned = CharacterClassContents<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        match self {
            Self::CharacterClassRange(it) => {
                CharacterClassContents::CharacterClassRange(CloneIn::clone_in(it, allocator))
            }
            Self::CharacterClassEscape(it) => {
                CharacterClassContents::CharacterClassEscape(CloneIn::clone_in(it, allocator))
            }
            Self::UnicodePropertyEscape(it) => {
                CharacterClassContents::UnicodePropertyEscape(CloneIn::clone_in(it, allocator))
            }
            Self::Character(it) => {
                CharacterClassContents::Character(CloneIn::clone_in(it, allocator))
            }
            Self::NestedCharacterClass(it) => {
                CharacterClassContents::NestedCharacterClass(CloneIn::clone_in(it, allocator))
            }
            Self::ClassStringDisjunction(it) => {
                CharacterClassContents::ClassStringDisjunction(CloneIn::clone_in(it, allocator))
            }
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        match self {
            Self::CharacterClassRange(it) => CharacterClassContents::CharacterClassRange(
                CloneIn::clone_in_with_semantic_ids(it, allocator),
            ),
            Self::CharacterClassEscape(it) => CharacterClassContents::CharacterClassEscape(
                CloneIn::clone_in_with_semantic_ids(it, allocator),
            ),
            Self::UnicodePropertyEscape(it) => CharacterClassContents::UnicodePropertyEscape(
                CloneIn::clone_in_with_semantic_ids(it, allocator),
            ),
            Self::Character(it) => CharacterClassContents::Character(
                CloneIn::clone_in_with_semantic_ids(it, allocator),
            ),
            Self::NestedCharacterClass(it) => CharacterClassContents::NestedCharacterClass(
                CloneIn::clone_in_with_semantic_ids(it, allocator),
            ),
            Self::ClassStringDisjunction(it) => CharacterClassContents::ClassStringDisjunction(
                CloneIn::clone_in_with_semantic_ids(it, allocator),
            ),
        }
    }
}

impl<'alloc> CloneIn<'alloc> for CharacterClassRange {
    type Cloned = CharacterClassRange;
    fn clone_in(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        CharacterClassRange {
            span: CloneIn::clone_in(&self.span, allocator),
            min: CloneIn::clone_in(&self.min, allocator),
            max: CloneIn::clone_in(&self.max, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        CharacterClassRange {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            min: CloneIn::clone_in_with_semantic_ids(&self.min, allocator),
            max: CloneIn::clone_in_with_semantic_ids(&self.max, allocator),
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for ClassStringDisjunction<'_> {
    type Cloned = ClassStringDisjunction<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        ClassStringDisjunction {
            span: CloneIn::clone_in(&self.span, allocator),
            strings: CloneIn::clone_in(&self.strings, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        ClassStringDisjunction {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            strings: CloneIn::clone_in_with_semantic_ids(&self.strings, allocator),
            body: CloneIn::clone_in_with_semantic_ids(&self.body, allocator),
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for ClassString<'_> {
    type Cloned = ClassString<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        ClassString {
            span: CloneIn::clone_in(&self.span, allocator),
            strings: CloneIn::clone_in(&self.strings, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        ClassString {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            strings: CloneIn::clone_in_with_semantic_ids(&self.strings, allocator),
            body: CloneIn::clone_in_with_semantic_ids(&self.body, allocator),
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for CapturingGroup<'_> {
    type Cloned = CapturingGroup<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        CapturingGroup {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        CapturingGroup {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            name: CloneIn::clone_in_with_semantic_ids(&self.name, allocator),
            body: CloneIn::clone_in_with_semantic_ids(&self.body, allocator),
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for IgnoreGroup<'_> {
    type Cloned = IgnoreGroup<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        IgnoreGroup {
            span: CloneIn::clone_in(&self.span, allocator),
            modifiers: CloneIn::clone_in(&self.modifiers, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        IgnoreGroup {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            modifiers: CloneIn::clone_in_with_semantic_ids(&self.modifiers, allocator),
            body: CloneIn::clone_in_with_semantic_ids(&self.body, allocator),
        }
    }
}

impl<'alloc> CloneIn<'alloc> for Modifiers {
    type Cloned = Modifiers;
    fn clone_in(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        Modifiers {
            span: CloneIn::clone_in(&self.span, allocator),
            enabling: CloneIn::clone_in(&self.enabling, allocator),
            disabling: CloneIn::clone_in(&self.disabling, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        Modifiers {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            enabling: CloneIn::clone_in_with_semantic_ids(&self.enabling, allocator),
            disabling: CloneIn::clone_in_with_semantic_ids(&self.disabling, allocator),
        }
    }
}

impl<'alloc> CloneIn<'alloc> for Modifier {
    type Cloned = Modifier;
    fn clone_in(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        Modifier {
            ignore_case: CloneIn::clone_in(&self.ignore_case, allocator),
            multiline: CloneIn::clone_in(&self.multiline, allocator),
            sticky: CloneIn::clone_in(&self.sticky, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        Modifier {
            ignore_case: CloneIn::clone_in_with_semantic_ids(&self.ignore_case, allocator),
            multiline: CloneIn::clone_in_with_semantic_ids(&self.multiline, allocator),
            sticky: CloneIn::clone_in_with_semantic_ids(&self.sticky, allocator),
        }
    }
}

impl<'alloc> CloneIn<'alloc> for IndexedReference {
    type Cloned = IndexedReference;
    fn clone_in(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        IndexedReference {
            span: CloneIn::clone_in(&self.span, allocator),
            index: CloneIn::clone_in(&self.index, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'alloc Allocator) -> Self::Cloned {
        IndexedReference {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            index: CloneIn::clone_in_with_semantic_ids(&self.index, allocator),
        }
    }
}

impl<'new_alloc> CloneIn<'new_alloc> for NamedReference<'_> {
    type Cloned = NamedReference<'new_alloc>;
    fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        NamedReference {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
        }
    }

    fn clone_in_with_semantic_ids(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
        NamedReference {
            span: CloneIn::clone_in_with_semantic_ids(&self.span, allocator),
            name: CloneIn::clone_in_with_semantic_ids(&self.name, allocator),
        }
    }
}
