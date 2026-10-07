# Collaboration Rules — BeepEngine

## Learning objective

BeepEngine is an independent audio project written in Rust. Its development is an opportunity to learn Rust progressively. The first milestone is to generate and play a simple beep, followed by a synthesized crash sound. PHW is a potential first consumer, not a constraint on the architecture.

## Project language

- Use English for all project content, including documentation, code, comments, identifiers, and commit messages.
- Conversations with the user may remain in French. Explain proposed code in French when appropriate, while keeping the code itself and project content in English.

## Allowed changes

- The assistant may create and edit only files with the `.md` extension, for project documentation.
- The assistant must not write any code in the repository on the user's behalf.
- All proposed code must be shown in the chat: the user copies and writes it themselves.
- This rule also applies to even the simplest corrections in any non-Markdown file: show the correction in the chat without editing the file.
- Do not bypass this restriction by using a command, tool, generator, or another agent to create or modify non-Markdown files.
- Code examples in Markdown documentation are allowed, but must not be automatically extracted or applied to code files.

## Commits

- The user makes commits themselves. The assistant must not create commits on their behalf.
- The assistant suggests commit messages in the chat using the Conventional Commits format: `type(scope): message`.
- Example: `feat(audio): add startup beep`.
- Choose a type and scope that match the actual changes, such as `feat`, `fix`, or `docs`.
- Do not stage changes, push, or perform any other Git write operation on the user's behalf.

## Teaching approach

- Explain why a Rust concept, dependency, or technical decision is needed before introducing it.
- Prefer the simplest solution that helps the user understand the underlying concept.
- Introduce complexity progressively and let the user write meaningful parts of the implementation.
- Show proposed code in the chat with its intended location and the explanations needed to understand it.
- Do not generate a sophisticated architecture in anticipation of future needs.

## Initial scope

Start with a program that plays a beep, then explore a simple crash sound, such as a descending pitch. Do not add MIDI, music, mixing, effects, spatial audio, or integration with PHW or the game engine without an explicit need.

PHW-specific behavior (timing, loading screens, logs, jokes, and visuals) belongs in PHW. BeepEngine produces sounds and remains reusable.
