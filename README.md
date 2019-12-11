# Clip

A small command-line utility which allows you to:

1. Pass input to the tool for storage (hint: pipeable!)
2. Retrieve that input when it is invoked with no arguments

## Usage

```bash
pwd |clip
```

```bash
cd $(clip)
```

or
```bash
cd clip -o
```

## Implementation
Input is stored in file in /var/lib/clip/clipstorage
