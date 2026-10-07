# VSA ecosystem roadmap

All four products are Planned metadata in CORE. Bundled manifests describe names and identities only, with no reviewed permission declarations or product implementations. The Rust SDK and inert automation contracts are experimental foundations. Integration depends on reviewed module admission, compatibility, action contracts, consent, and an enforceable runtime boundary. No product has an install/enable action today.

Permission candidates below are design questions, **not grants or final declarations**. Requirements must be narrowed by feature and resource scope before admission. Do not give a whole product every candidate permission. CORE's logical permission policy is not an OS sandbox.

## Steam Power Suite

Identity: `steam-power-suite`.

Potential boundaries: profile metadata tools, local inventory data utilities, market information, account selection metadata, privacy controls, status/custom-game metadata, and a separately reviewed browser-extension protocol. First useful work should accept explicit local sample data and provide pure transformations without authentication or scraping.

Separate public metadata, local user-provided data, authenticated account operations, and extension messages. An account selector must eventually use opaque account references, with credentials owned by a reviewed Vault/provider, never settings or manifests. An extension must not inherit CORE privileges or pass arbitrary commands/URLs.

Review network, scoped filesystem reads/writes, and notifications per feature. Account switching and multi-account handling remain blocked on authentication and secret-lifetime review. No Steam login, Steam Guard, credential collection, automated trading, cheating, VAC bypass, or impersonation is implemented or authorized by this foundation.

## VSA ASF

Identity: `vsa-asf`.

Future boundaries: validated nonsecret configuration, opaque bot identities, per-bot status, and a versioned adapter to an independently isolated runtime. Start with fixture-based parsing and status translation after documenting the actual upstream schema and license. No ASF source is imported here.

Before reuse or redistribution, inspect the exact upstream revision's license, copyright, notices, dependency licenses, and derivative/distribution obligations. CORE's GPL-3.0-only license does not substitute for that review or relicense upstream code.

A future process adapter requires fixed approved binaries, integrity checks, restricted arguments, lifecycle ownership, resource isolation, and least privilege. A web panel needs a reviewed origin/authentication boundary; embedding arbitrary remote pages in CORE is not acceptable. Review process execution, network, scoped configuration storage, and notifications only when a concrete adapter exists. Multi-bot credentials and Steam Guard are deferred to owner security review.

## VSA StreamDropCollector

Identity: `vsa-stream-drop-collector`.

Future boundaries: campaign discovery metadata, provider-specific adapters for Twitch/Kick, progress snapshots, explicit watch/claim requests, and generic campaign contracts. A progress contract should identify provider/campaign with opaque IDs, represent unknown totals explicitly, validate nonnegative bounded counts, and distinguish observed progress from successful claims. CORE must never display a successful claim before provider confirmation.

Provider adapters must own their authentication boundary and handle revocation, rate limits, consent, and errors without exposing tokens in IPC. Browser automation requires separate isolation and permission review. No browser profile, cookies, Twitch/Kick tokens, authenticated watching, claiming, or browser automation is implemented.

Potential capabilities are network, notifications, and a narrowly designed browser capability if justified by a real adapter. CORE does not add that capability today. Pure progress validation against synthetic fixtures is an appropriate future first module milestone once the provider contract is reviewed.

## VSA R4R + SDA

Identity: `vsa-r4r-sda`.

Future boundaries: separate nonsecret preferences from account/device references, bounded status snapshots, and explicitly selected safe action IDs. Scheduled requests should use the inert automation contract only after an action registry and permission broker exist. No secret keys, account login, trading automation, or scheduler execution is implemented.

Review upstream ownership/licensing and the exact combined product scope before creating adapters. Potential network, notifications, and secret-access requirements remain undecided. Secret access must be scoped by account/purpose and available only through a reviewed provider; a blanket `secrets.read` label is insufficient enforcement.

## Integration milestones

1. Owner reviews the metadata SDK, schema evolution policy, product scope, and native CORE behavior.
2. Define a pure first module feature using synthetic/user-provided nonsecret fixtures, with tests and reviewed licensing.
3. Specify version compatibility, module-defined action IDs, health/status/error contracts, and explicit resource-scoped consent.
4. Review provenance, signed packages, revocation, platform process isolation, and broker enforcement before any third-party execution.
5. Review each authenticated provider and Vault boundary independently before real accounts or secrets are involved.

No date or production-readiness promise is made. Installation, Module Store, automatic downloads, execution, OS sandboxing, and all authenticated products remain absent.
