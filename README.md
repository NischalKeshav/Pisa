System Dependencies:
bash-preexec

Generate the Bash activation scripts with `cargo run`, then activate the
`(Pisa)` terminal prompt in your current shell:

```bash
source .pisa/activate
```

Restore your original prompt with `deactivate` or `source .pisa/deactivate`.
These scripts change the prompt only; they do not create a Python virtual environment.
