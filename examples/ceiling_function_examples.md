# CEILING Function Examples

The CEILING function rounds a number UP to the nearest integer or to the nearest multiple of significance.

## Syntax

```
CEILING(number)
CEILING(number, significance)
```

## Parameters

- **number**: The value to round up (required)
- **significance**: The multiple to which to round (optional, defaults to 1)

## Basic Usage

### Simple Ceiling (No Significance)

```
=CEILING(4.3)        → 5
=CEILING(4.8)        → 5
=CEILING(5)          → 5
=CEILING(-4.3)       → -4 (rounds toward zero)
=CEILING(0)          → 0
```

### With Significance Parameter

```
=CEILING(4.3, 0.5)   → 4.5    (rounds to nearest 0.5)
=CEILING(12, 5)      → 15     (rounds to nearest 5)
=CEILING(12.7, 5)    → 15     (rounds to nearest 5)
=CEILING(1.2, 0.1)   → 1.2    (rounds to nearest 0.1)
=CEILING(1.23, 0.1)  → 1.3    (rounds to nearest 0.1)
```

### With Negative Numbers

```
=CEILING(-4.3)       → -4     (rounds toward zero)
=CEILING(-4.8)       → -4     (rounds toward zero)
=CEILING(-5)         → -5
=CEILING(-12.7, -5)  → -15    (rounds away from zero)
```

## Unit-Aware Examples

### Preserving Units

The CEILING function preserves the unit from the first argument:

```
=CEILING(4.3m)                → 5m
=CEILING(12.7USD)             → 13USD
=CEILING(25.3C)               → 26C
=CEILING(1234.5GB)            → 1235GB
```

### With Dimensionless Significance

When significance is dimensionless, the result preserves the input unit:

```
=CEILING(12.7m, 5)            → 15m
=CEILING(1234.56USD, 100)     → 1300USD
=CEILING(98.2kg, 10)          → 100kg
```

### With Compatible Units

Both arguments can have units if they're compatible:

```
=CEILING(100cm, 0.5m)         → 100cm    (1m rounded to nearest 0.5m = 1m)
=CEILING(150cm, 1m)           → 200cm    (1.5m rounded to nearest 1m = 2m)
=CEILING(1.5km, 500m)         → 1.5km    (already at multiple)
=CEILING(1.7km, 500m)         → 2km      (rounds up to 2km)
```

### Currency Rounding

Useful for rounding prices to standard increments:

```
=CEILING(12.34USD, 0.25USD)   → 12.50USD  (round to nearest quarter)
=CEILING(99.99USD, 1USD)      → 100USD    (round to nearest dollar)
=CEILING(47.23EUR, 5EUR)      → 50EUR     (round to nearest 5 euros)
```

## Error Cases

### Incompatible Units

```
=CEILING(10m, 5kg)            → Error: Incompatible units
=CEILING(100USD, 10s)         → Error: Incompatible units
```

### Invalid Arguments

```
=CEILING()                    → Error: Requires 1 or 2 arguments
=CEILING(1, 2, 3)             → Error: Too many arguments
=CEILING("hello")             → Error: Requires numeric argument
=CEILING(10, 0)               → Error: Significance cannot be zero
```

## Practical Use Cases

### Budget Planning

Round costs to standard billing increments:

```
=CEILING(123.45USD, 100USD)   → 200USD    (round to nearest $100)
=CEILING(1234.56USD, 1000USD) → 2000USD   (round to nearest $1000)
```

### Inventory Management

Round to case quantities:

```
=CEILING(47, 12)              → 48        (4 cases of 12)
=CEILING(100, 24)             → 120       (5 cases of 24)
```

### Time Rounding

Round duration to billing increments:

```
=CEILING(23min, 15min)        → 30min     (round to nearest 15 minutes)
=CEILING(1.3hr, 0.5hr)        → 1.5hr     (round to nearest half hour)
```

### Storage Allocation

Round to block sizes:

```
=CEILING(1234MB, 512MB)       → 1536MB    (3 blocks of 512MB)
=CEILING(3.2GB, 1GB)          → 4GB       (4 blocks of 1GB)
```

## Advanced Examples

### Nested Functions

```
=CEILING(CEILING(4.1))        → 5
=CEILING(4.3) + 5             → 10        (5 + 5)
=CEILING(4.3 + 0.5)           → 5
```

### In Formulas

```
// Calculate total cases needed
=CEILING(A1, 12)              // Where A1 = units needed

// Round up time to billing increment
=CEILING(B2hr, 0.25hr)        // Where B2 = actual hours

// Price rounding
=CEILING(C3USD, 0.99USD)      // Where C3 = calculated price
```

## Comparison with Related Functions

### CEILING vs FLOOR

```
CEILING(4.3)    → 5     // Rounds UP
FLOOR(4.3)      → 4     // Rounds DOWN
```

### CEILING vs ROUND

```
CEILING(4.3)    → 5     // Always rounds UP
ROUND(4.3)      → 4     // Rounds to nearest (down in this case)
ROUND(4.6)      → 5     // Rounds to nearest (up in this case)
```

## Notes

- CEILING always rounds UP (away from zero for positive numbers, toward zero for negative numbers)
- The result unit always matches the first argument's unit
- If significance is omitted, it defaults to 1
- Significance cannot be zero
- Both arguments must have compatible units (same dimension) or one must be dimensionless
- Negative significance is allowed and produces mathematically correct results
