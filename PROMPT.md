Initialise le projet Burnr from scratch.

Contexte : app desktop native Tauri v2 (Rust backend + React frontend) qui lit des fichiers JSONL locaux de Claude Code et Codex pour afficher des stats de tokens. 100% local, aucune donnée envoyée en ligne. Site vitrine séparé sur burnrapp.xyz.

Crée la structure suivante :

1. `.gitignore` adapté pour un projet Tauri v2 (Rust + Node/React)

2. `README.md` avec :
   - Présentation du projet Burnr
   - Stack technique (Tauri v2, Rust, React)
   - Instructions d'installation à venir
   - Statut : Work in Progress

3. `ROADMAP.md` avec les features organisées en 3 phases :
   MVP (v1) : lecture JSONL locale Claude Code + Codex, stats globales (input/output/cache/coût), heatmap 12 mois, breakdown par modèle et projet, limites Claude Code temps réel + reset timers, dark mode natif
   v2 : support Cursor/Windsurf/Gemini CLI, détection source (abonnement/API/Bedrock), alertes budget, export CSV/PDF, streaks, graphe évolution coût, filtres période, comparaison flexible app×modèle×période
   Nice-to-have : widget menubar, thème custom couleur d'accent

4. `TODO.md` avec les tâches immédiates :
   - Initialiser le projet Tauri v2
   - Choisir et setup le frontend React
   - Explorer la structure des fichiers JSONL Claude Code et Codex
   - Implémenter le parser Rust
   - Définir les commandes Tauri (IPC frontend ↔ backend)

5. `CHANGELOG.md` vide avec juste le header et une entrée `[Unreleased]`

6. `CONTRIBUTING.md` basique (pour plus tard)

7. `.github/ISSUE_TEMPLATE/` avec deux templates : bug_report et feature_request

8. `LICENSE` fichier avec la license MIT, nom : h41k0, année 2026

Ne touche pas encore à la structure Tauri, juste ces fichiers de base pour bien démarrer le projet.