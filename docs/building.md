# Building

## Desktop

```bash
just build almond
```

## Android

```bash
just android build-apk   # APK (production)
just android build-aab   # AAB
```

## Docs Site

```bash
just build-docs
just preview-docs   # preview the production build locally
```

## Lint Before Building

```bash
just lint all
```

## Clean Artifacts

```bash
just clean all       # clean everything
just clean almonds   # frontend only
just clean kernel    # kernel crate only
just clean orchard   # orchard crate only
```
