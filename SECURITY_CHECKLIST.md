# ✅ Checklist de Seguridad Pre-GitHub

## Antes de hacer el primer commit:

- [x] Sin credenciales en código fuente
- [x] Sin API keys o tokens
- [x] Sin paths personales en documentación
- [x] .gitignore configurado
- [x] Binarios excluidos
- [x] Licencia MIT incluida
- [x] Disclaimer educativo presente
- [x] Código limpio (0 warnings)
- [x] Tests pasando
- [ ] Revisar con: `git status`
- [ ] Revisar con: `git diff`

## Comandos de verificación antes de push:

```bash
# 1. Verificar que no haya archivos sensibles
git status

# 2. Ver todos los cambios que se van a commitear
git diff --cached

# 3. Buscar patterns sensibles
git grep -i "password\|secret\|token\|api_key"

# 4. Verificar .gitignore
cat .gitignore
```

## Archivos que NO deben ir a GitHub:

- target/ (binarios)
- Cargo.lock
- GITHUB_READY.md (notas internas)
- SUMMARY.md (opcional)
- *.log
- .jetbrains-trial-backups/

## Todo listo ✅
