# BigBother Age Verification Protocol v1

OS-level age bracket attestation over D-Bus, per Colorado SB26-051 and California AB-1043.

## Bus & Address

| Field | Value |
|-------|-------|
| Bus | System bus (`org.freedesktop.DBus.System`) |
| Interface | `org.bigbother.AgeAttestation1` |
| Object path | `/org/bigbother/AgeAttestation1` |

## Age Brackets

The service uses four age brackets. All methods and signals use these string values:

| Value | Age range |
|-------|-----------|
| `under_13` | 0-12 |
| `13_to_15` | 13-15 |
| `16_to_17` | 16-17 |
| `18_plus` | 18+ |

## Methods

### `GetAgeBracket() -> s`

Returns the age bracket string for the current user, or an empty string `""` if no attestation is on file.

**Example (busctl):**

```sh
busctl --system call org.bigbother.AgeAttestation1 \
  /org/bigbother/AgeAttestation1 \
  org.bigbother.AgeAttestation1 \
  GetAgeBracket
# Returns: s "18_plus"
```

### `SetAge(y age) -> s`

Accepts a self-attested age as a `byte` (0-255), maps it to a bracket, stores it, and returns the resulting bracket string. Emits `AgeBracketChanged` on success.

| Parameter | Type | Description |
|-----------|------|-------------|
| `age` | `y` (byte) | Self-attested age in years |
| Return | `s` (string) | The resulting age bracket |

**Example (busctl):**

```sh
busctl --system call org.bigbother.AgeAttestation1 \
  /org/bigbother/AgeAttestation1 \
  org.bigbother.AgeAttestation1 \
  SetAge y 25
# Returns: s "18_plus"
```

## Signals

### `AgeBracketChanged(s bracket)`

Emitted when a user's age bracket changes via `SetAge`.

| Parameter | Type | Description |
|-----------|------|-------------|
| `bracket` | `s` (string) | The new age bracket value |

**Subscribe (busctl):**

```sh
busctl --system monitor org.bigbother.AgeAttestation1
```

## Storage

Attestation data is stored encrypted per-user at:

```
$XDG_DATA_HOME/bb-age-attestation/attestation.age
```

This file is encrypted with the `age` encryption tool. Applications must not read this file directly; always use the D-Bus interface.

## Error Handling

| Scenario | Behavior |
|----------|----------|
| No attestation on file | `GetAgeBracket` returns `""` |
| Decryption failure | `GetAgeBracket` returns D-Bus error `org.freedesktop.DBus.Error.Failed` |
| Storage failure | `SetAge` returns D-Bus error `org.freedesktop.DBus.Error.Failed` |
| Service not running | D-Bus returns `org.freedesktop.DBus.Error.ServiceUnknown` |
