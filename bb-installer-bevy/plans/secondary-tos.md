# Plan: Secondary Terms of Service

Status: planning. Comes after the real warning, before `Sign`.

## Core idea (decided)

The user gets a huge ToS. It opens like a normal one and then turns into an
enormous lorem ipsum. They scroll to the bottom and press Continue. We then show
how long they took to "read" it, which is impossibly fast for a human, and
something follows from that: a captcha, being mocked for not reading, or a
minigame.

## Proposed flow

Each step leads to the next, and the whole thing runs on one premise:
**reading that fast means you are not a person.**

1. **The wall of text**: the document, read by scrolling.
1. **Reading report**: the reading statistics, delivered as a finding.
1. **Bot suspicion → verification**: nobody reads that fast, so the user must
   prove they are human.
1. **Remedial material**: when they fail, an illustrated version of the ToS for
   people who do not read (the stickmen).
1. **The consent interview**: the old free-will script
   (`git show 7fddb03^:bb-installer-bevy/src/tos.rs`).
1. → `Sign`.

Steps 3–5 can be cut or reordered. Steps 1–2 are the core.

## 1. The wall of text

### Content

- **The normal part.** It opens with the ordinary legal text from the current
  `TOS_TEXT`, so the first screen looks legitimate.

- **The transition.** It slides into Latin gradually rather than switching. A
  few paragraphs of legal sentences start picking up Latin words ("The
  licensee shall indemnify the dolor sit amet…"), until it is entirely lorem
  ipsum.

- **Structure.** Keep numbered clauses all the way down ("§ 212.4(c)"), so the
  lorem ipsum still looks like legal text. Section headings stay in English
  and stay plausible ("Limitation of Liability", "Arbitration",
  "Severability").

- **Size.** About 40,000 words, enough for a "this would take you ~3 hours"
  statistic. Generate it from a seed, not a checked-in blob, so it costs
  nothing.

- **Hidden clauses.** A few real sentences buried in the lorem ipsum, which
  later steps can refer to. For example:

  - "If you have read this far, click the eye instead of Continue."
  - "By reading this sentence you confirm you read all previous sentences."
  - "The user acknowledges that § 1 through § 211 were in Latin."

  They are the sincere twist: everything said really was in the document.

### Making the scrolling strange (pick some)

These build on what already exists: inverted scrolling, `CursorAtraction`, and
`ScrollPosition` being writable.

- **Speed camera.** Scrolling faster than a reading speed limit flashes the eye
  and issues a "Reading speed violation" ticket: "38,400 wpm in a 240 wpm
  zone." Tickets pile up and are itemised in the report. It never stops the
  user. It only records.
- **Repelling scrollbar.** The scrollbar thumb is tiny because the document is
  huge, and it pushes the cursor away (`CursorAtraction`), so you have to use
  the wheel.
- **Terms updated while reading.** Near the bottom, new amendments get appended
  ("Amendment 3, effective immediately"). Two or three times at most, then it
  ends.
- **Shrinking fine print.** The font gets smaller as you go. The last pages are
  close to unreadable.
- **The last pixel.** At the bottom, Continue stays disabled with "Please
  scroll to the bottom", until one more scroll tick that moves nothing.
- **Scroll momentum.** Wheel input becomes velocity with friction, so long
  flings overshoot and drift.

## 2. Reading report

Shown after Continue, in a flat official tone with no commentary:

```
Reading report
  Document length       41,206 words
  Time spent reading    0:00:07
  Reading speed         353,194 words per minute
  Average adult         about 240 words per minute
  Time a human needs    2 h 51 min
  Speed violations      14
```

- Measure from the moment the user first scrolls to the moment they press
  Continue.
- If they really did read slowly (time above some threshold, say over an hour
  of an idle installer), say it differently: "Reading speed within human
  range. This is unusual and has been flagged."

## 3. Verification

The report's conclusion: "Reading speed is not consistent with a human.
Verification required." Options:

- **Comprehension check.** "Which of the following appears in § 212.4(c)?" Four
  Latin phrases that all look the same. Or ask about one of the hidden clauses
  with plain English options.
- **CAPTCHA.** "Select all squares containing terms you agreed to." Every
  square is lorem ipsum.
- **Hidden clause.** If they find and act on "click the eye instead of
  Continue", skip straight to the interview with "Reading confirmed." The only
  way to pass is to have actually read it.

Failing is expected and must still lead forward.

## 4. Remedial material (stickmen)

An "illustrated summary" of the ToS for users who did not read it. It is
presented as an accessibility feature, not as an insult:

- A series of hand-drawn stickman panels with arrows and short captions, e.g.
  a stickman at a computer with an arrow and "this is you", then "this is the
  text", then "you did not read it".
- Captions in lowercase only, which calls back to the distro's lowercase font.
- One panel per click. The last one ends on a checkbox: "I have understood the
  illustrated summary."

Assets: hand-drawn PNGs in `assets/tos/` (to be drawn). Drawn badly on purpose
is fine; they should look like someone in compliance drew them.

## 5. Consent interview

Port the removed free-will script (see `CLAUDE.md`, "The secondary ToS"). It
fits here because the user has just been told they cannot read, so the
interview reads as the follow-up.

## Technical notes

- **Scroll state.** `ScrollArea` (bevy_ui_widgets) only clamps
  `ScrollPosition` to `content_size - visible_size`. A system reading
  `ScrollPosition` plus `ComputedNode` each frame gives position, speed (for
  the speed camera) and at-bottom detection. Writing `ScrollPosition` gives
  momentum, pushback and so on. Remember to account for
  `inverse_scale_factor`, as `scrollarea_on_scroll` does.
- **Text size.** 40k words as one `Text` is probably too heavy for layout,
  especially on lavapipe in the VM test. Split it into one node per clause.
  If that's still slow, only keep the clauses near the viewport and use
  spacers for the rest, which is easy because the document is generated.
- **Word count.** Count from the generated paragraphs, not the layout.
- **Reading-time state.** A resource (`ReadingReport`) filled while
  scrolling. Later pages (the end-of-install summary) can call back to it.
- **Interview engine.** The deleted `dialogue.rs` and `script.rs` can be
  restored from `7fddb03^` and moved onto feathers widgets.
- **States.** Probably sub-states of `TermsOfService` (`Reading`, `Report`,
  `Verify`, `Remedial`, `Interview`), or one page with its own internal stage
  enum.

## Open questions

- Does verification always fail, or can a user pass by actually finding the
  hidden clause?
- Which scroll oddities, and how many? More than two or three will get tiring
  on a page that is already long.
- Who draws the stickmen, and how many panels?
- Is the reading report its own screen, or an overlay on the document?
