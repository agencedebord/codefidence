# When to use codefidence

## When codefidence works well

- **TypeScript/JavaScript backend projects with business logic** — services with billing rules, access control exceptions, domain-specific calculations. The kind of code where "why is this like that?" matters more than "what does this do?"
- **Projects with undocumented exceptions and legacy code** — workarounds nobody remembers, client-specific overrides, intentional deviations from the norm. The wiki captures these before they get "fixed" by someone who didn't know.
- **Teams where AI assistants edit code (Claude Code, Cursor, etc.)** — AI tools follow patterns and conventions. Without documented exceptions, they will normalize your intentional weirdness. The wiki gives them context.
- **Repos with 10-100+ files where implicit knowledge accumulates** — big enough that no one person holds the full picture, small enough that a heavyweight docs system is overkill.

## When it's less useful

- **Very small projects (< 10 files)** — you can keep it all in your head. The overhead of maintaining a wiki exceeds the benefit.
- **Pure UI/CSS projects** — few business rules to track. The "decisions" that matter are visual, not logical.
- **Greenfield projects with no legacy decisions yet** — nothing to document. Start the wiki when the first non-obvious decision lands, not on day one.
- **Projects that already have thorough ADR documentation** — if your team already writes Architecture Decision Records and keeps them current, codefidence adds marginal value. It's designed for teams that don't write ADRs.

## Best fit

A TypeScript backend with 2-5 developers, some legacy code, and an AI assistant that edits files. The wiki prevents the AI from "fixing" intentional exceptions and gives new team members context that would otherwise live only in someone's head.
