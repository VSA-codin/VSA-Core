# Accessibility review and native QA

Status: source review performed; no NVDA, JAWS, VoiceOver or native assistive-technology certification.

The shell uses a labeled navigation landmark, `aria-current`, main landmark/heading, skip link, labeled window buttons and explicit status/error text. Module dropdown exposes listbox/options, expanded/selected/active descendant states and keyboard navigation. Settings uses a labeled switch; destructive recovery requires review and confirmation. This sprint adds Cancel/Escape focus return and focuses successful save/recovery status. Window-control failures use fixed accessible error text. Support preview has a labeled selectable textarea and focus return on Hide. Decorative icons are hidden from accessibility APIs.

Existing CSS includes visible focus, reduced-motion and forced-colors adaptations, responsive layouts and compact spacing. Source review cannot establish rendered contrast, physical focus visibility or screen-reader behavior.

For Windows and Linux native webviews:

- [ ] Record OS/webview/reader/version, window size, zoom, scale and compact state.
- [ ] Tab/Shift+Tab through titlebar, skip link, navigation and every page; no unreachable or trapped controls.
- [ ] Skip link places focus in main; heading order and landmarks are meaningful. Navigation announces current page without excessive duplicate speech.
- [ ] Dropdown: Enter/Space/open, arrows, Home/End, initial-letter navigation, selected state, Escape focus return, forward/backward Tab, pointer selection and outside dismissal.
- [ ] Search label, result count, no-result/clear states and details disclosures announce meaningfully.
- [ ] Switch announces name/state; save/loading/error/retry/recovery remain accessible with no unexpected navigation.
- [ ] Recovery review/confirmation text is understandable; Cancel and Escape restore focus, success announces/focuses status, failure preserves access to retry/review.
- [ ] Hide support preview restores focus; long selectable output works at zoom without clipping.
- [ ] Forced colors/high contrast, reduced motion, OS light/dark, minimum size and 200% zoom remain usable; status meaning does not rely on color.
- [ ] Check computed foreground/background contrast and focus contrast in actual webviews. Inspect horizontal/vertical overflow and mixed DPI monitors.
- [ ] NVDA/JAWS on Windows and a suitable Linux reader: test landmarks, labels, headings, live regions, listbox active selection and keyboard flows; record observed failures explicitly.

Manual Windows baseline titlebar operation is recorded separately in [Windows QA](WINDOWS_QA.md); it is not screen-reader testing.
