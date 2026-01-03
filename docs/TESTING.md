# Cahier de Tests - Notion Killer

Ce document permet de valider le bon fonctionnement de l'application Notion Killer.

---

## Informations de test

| Champ | Valeur |
|-------|--------|
| **Testeur** | |
| **Date** | |
| **Version** | |
| **Navigateur** | |
| **OS** | |

---

## 1. Authentification

### 1.1 Inscription

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 1.1.1 | Inscription réussie | 1. Aller sur `/register`<br>2. Remplir nom, email, mot de passe<br>3. Cliquer "S'inscrire" | Redirection vers le dashboard | ☐ |
| 1.1.2 | Email déjà utilisé | 1. S'inscrire avec un email existant | Message d'erreur "Email déjà utilisé" | ☐ |
| 1.1.3 | Mot de passe trop court | 1. Entrer un mot de passe < 8 caractères | Message d'erreur de validation | ☐ |
| 1.1.4 | Champs obligatoires | 1. Soumettre le formulaire vide | Messages d'erreur sur les champs requis | ☐ |

### 1.2 Connexion

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 1.2.1 | Connexion réussie | 1. Aller sur `/login`<br>2. Entrer email/mot de passe valides<br>3. Cliquer "Se connecter" | Redirection vers le dashboard | ☐ |
| 1.2.2 | Mauvais mot de passe | 1. Entrer un mot de passe incorrect | Message "Identifiants incorrects" | ☐ |
| 1.2.3 | Email inexistant | 1. Entrer un email non inscrit | Message "Identifiants incorrects" | ☐ |
| 1.2.4 | Persistance de session | 1. Se connecter<br>2. Fermer l'onglet<br>3. Rouvrir l'application | Toujours connecté | ☐ |

### 1.3 Déconnexion

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 1.3.1 | Déconnexion | 1. Cliquer sur le bouton de déconnexion | Retour à la page de login | ☐ |
| 1.3.2 | Accès protégé après déconnexion | 1. Se déconnecter<br>2. Accéder à une URL protégée | Redirection vers login | ☐ |

---

## 2. Workspaces

### 2.1 Gestion des workspaces

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 2.1.1 | Créer un workspace | 1. Cliquer "Nouveau workspace"<br>2. Entrer un nom<br>3. Valider | Workspace créé et affiché dans la sidebar | ☐ |
| 2.1.2 | Renommer un workspace | 1. Cliquer sur les options du workspace<br>2. Choisir "Renommer"<br>3. Entrer le nouveau nom | Nom mis à jour | ☐ |
| 2.1.3 | Supprimer un workspace | 1. Cliquer sur les options<br>2. Choisir "Supprimer"<br>3. Confirmer | Workspace supprimé | ☐ |
| 2.1.4 | Changer de workspace | 1. Cliquer sur un autre workspace | Affichage des pages du workspace sélectionné | ☐ |

---

## 3. Pages

### 3.1 Création de pages

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 3.1.1 | Créer une page | 1. Cliquer "Nouvelle page"<br>2. Entrer un titre | Page créée et ouverte | ☐ |
| 3.1.2 | Créer une sous-page | 1. Survoler une page dans la sidebar<br>2. Cliquer sur "+"<br>3. Entrer un titre | Sous-page créée sous la page parent | ☐ |
| 3.1.3 | Page sans titre | 1. Créer une page sans titre | Titre par défaut "Sans titre" | ☐ |

### 3.2 Navigation

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 3.2.1 | Ouvrir une page | 1. Cliquer sur une page dans la sidebar | Page affichée dans l'éditeur | ☐ |
| 3.2.2 | Fil d'Ariane (Breadcrumbs) | 1. Ouvrir une sous-page imbriquée | Breadcrumbs affichés avec la hiérarchie | ☐ |
| 3.2.3 | Naviguer via breadcrumbs | 1. Cliquer sur un élément du breadcrumb | Navigation vers la page cliquée | ☐ |

### 3.3 Gestion des pages

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 3.3.1 | Renommer une page | 1. Modifier le titre dans l'éditeur | Titre mis à jour dans la sidebar | ☐ |
| 3.3.2 | Dupliquer une page | 1. Menu contextuel > "Dupliquer" | Copie créée avec "(copie)" dans le titre | ☐ |
| 3.3.3 | Supprimer une page | 1. Menu contextuel > "Supprimer"<br>2. Confirmer | Page supprimée | ☐ |
| 3.3.4 | Déplacer une page (drag & drop) | 1. Glisser une page vers un autre emplacement | Page déplacée dans la hiérarchie | ☐ |

---

## 4. Éditeur de blocs

### 4.1 Saisie de texte

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 4.1.1 | Saisie simple | 1. Cliquer dans l'éditeur<br>2. Taper du texte | Texte affiché | ☐ |
| 4.1.2 | Nouveau paragraphe | 1. Appuyer sur Entrée | Nouveau paragraphe créé | ☐ |
| 4.1.3 | Saut de ligne | 1. Appuyer sur Shift+Entrée | Saut de ligne dans le même paragraphe | ☐ |
| 4.1.4 | Placeholder | 1. Ouvrir une page vide | Placeholder "Tapez '/' pour les commandes..." visible | ☐ |

### 4.2 Menu slash (/)

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 4.2.1 | Ouvrir le menu | 1. Taper `/` | Menu de commandes affiché | ☐ |
| 4.2.2 | Filtrer les commandes | 1. Taper `/h1` | Seuls les résultats correspondants affichés | ☐ |
| 4.2.3 | Sélectionner avec clavier | 1. Naviguer avec ↑↓<br>2. Appuyer Entrée | Bloc inséré | ☐ |
| 4.2.4 | Sélectionner avec souris | 1. Cliquer sur une commande | Bloc inséré | ☐ |
| 4.2.5 | Fermer le menu | 1. Appuyer Échap | Menu fermé | ☐ |

### 4.3 Types de blocs

| # | Scénario | Commande | Résultat attendu | OK |
|---|----------|----------|------------------|:--:|
| 4.3.1 | Texte | `/text` | Paragraphe normal | ☐ |
| 4.3.2 | Titre 1 | `/h1` ou `# ` | Grand titre | ☐ |
| 4.3.3 | Titre 2 | `/h2` ou `## ` | Titre moyen | ☐ |
| 4.3.4 | Titre 3 | `/h3` ou `### ` | Petit titre | ☐ |
| 4.3.5 | Liste à puces | `/bullet` ou `- ` | Liste non ordonnée | ☐ |
| 4.3.6 | Liste numérotée | `/numbered` ou `1. ` | Liste ordonnée | ☐ |
| 4.3.7 | Todo | `/todo` ou `[] ` | Case à cocher | ☐ |
| 4.3.8 | Citation | `/quote` ou `> ` | Bloc de citation | ☐ |
| 4.3.9 | Code | `/code` | Bloc de code avec coloration syntaxique | ☐ |
| 4.3.10 | Séparateur | `/divider` ou `---` | Ligne horizontale | ☐ |

### 4.4 Formatage de texte

| # | Scénario | Raccourci | Résultat attendu | OK |
|---|----------|-----------|------------------|:--:|
| 4.4.1 | Gras | Sélectionner + `Cmd+B` | Texte en **gras** | ☐ |
| 4.4.2 | Italique | Sélectionner + `Cmd+I` | Texte en *italique* | ☐ |
| 4.4.3 | Souligné | Sélectionner + `Cmd+U` | Texte souligné | ☐ |
| 4.4.4 | Barré | Sélectionner + `Cmd+Shift+S` | Texte ~~barré~~ | ☐ |
| 4.4.5 | Code inline | Sélectionner + `Cmd+E` | Texte en `code` | ☐ |
| 4.4.6 | Surligné | Sélectionner + `Cmd+Shift+H` | Texte surligné | ☐ |
| 4.4.7 | Lien | Sélectionner + `Cmd+K` | Dialogue d'ajout de lien | ☐ |

### 4.5 Barre d'outils flottante

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 4.5.1 | Affichage | 1. Sélectionner du texte | Barre d'outils apparaît au-dessus | ☐ |
| 4.5.2 | Masquage | 1. Cliquer ailleurs | Barre d'outils disparaît | ☐ |
| 4.5.3 | Formatage via toolbar | 1. Sélectionner du texte<br>2. Cliquer sur un bouton de formatage | Formatage appliqué | ☐ |

### 4.6 Listes

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 4.6.1 | Continuer la liste | 1. Être dans une liste<br>2. Appuyer Entrée | Nouvel élément de liste | ☐ |
| 4.6.2 | Quitter la liste | 1. Élément vide<br>2. Appuyer Entrée | Retour au paragraphe normal | ☐ |
| 4.6.3 | Indenter | 1. Appuyer Tab | Élément indenté | ☐ |
| 4.6.4 | Désindenter | 1. Appuyer Shift+Tab | Élément désindentée | ☐ |
| 4.6.5 | Cocher/décocher todo | 1. Cliquer sur la case | État inversé | ☐ |

### 4.7 Historique

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 4.7.1 | Annuler | 1. Modifier du contenu<br>2. Appuyer `Cmd+Z` | Modification annulée | ☐ |
| 4.7.2 | Rétablir | 1. Annuler<br>2. Appuyer `Cmd+Shift+Z` | Modification rétablie | ☐ |

---

## 5. Sauvegarde et synchronisation

### 5.1 Sauvegarde automatique

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 5.1.1 | Sauvegarde locale | 1. Modifier du contenu<br>2. Observer l'indicateur | "Saving..." puis "Saved" | ☐ |
| 5.1.2 | Persistance après refresh | 1. Modifier du contenu<br>2. Rafraîchir la page | Contenu préservé | ☐ |
| 5.1.3 | Persistance après fermeture | 1. Modifier du contenu<br>2. Fermer le navigateur<br>3. Rouvrir | Contenu préservé | ☐ |

### 5.2 Mode hors-ligne

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 5.2.1 | Édition hors-ligne | 1. Couper la connexion<br>2. Modifier du contenu | Édition fonctionne normalement | ☐ |
| 5.2.2 | Sync à la reconnexion | 1. Éditer hors-ligne<br>2. Rétablir la connexion | Contenu synchronisé | ☐ |

---

## 6. Interface utilisateur

### 6.1 Sidebar

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 6.1.1 | Afficher/masquer la sidebar | 1. Cliquer sur le bouton toggle | Sidebar s'affiche/se masque | ☐ |
| 6.1.2 | Arborescence des pages | 1. Créer des sous-pages | Hiérarchie visible avec indentation | ☐ |
| 6.1.3 | Expand/collapse | 1. Cliquer sur la flèche d'une page | Sous-pages affichées/masquées | ☐ |

### 6.2 Header de page

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 6.2.1 | Modifier le titre | 1. Cliquer sur le titre<br>2. Modifier<br>3. Cliquer ailleurs | Titre mis à jour | ☐ |
| 6.2.2 | Ajouter un emoji | 1. Cliquer sur "Add icon" | Picker d'emoji affiché | ☐ |
| 6.2.3 | Changer l'emoji | 1. Cliquer sur l'emoji existant | Picker pour changer | ☐ |

### 6.3 Responsive

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 6.3.1 | Vue mobile | 1. Réduire la fenêtre < 768px | Interface adaptée | ☐ |
| 6.3.2 | Vue tablette | 1. Réduire la fenêtre entre 768-1024px | Interface adaptée | ☐ |

---

## 7. Performance

| # | Scénario | Étapes | Résultat attendu | OK |
|---|----------|--------|------------------|:--:|
| 7.1 | Chargement initial | 1. Ouvrir l'application | Chargement < 3 secondes | ☐ |
| 7.2 | Changement de page | 1. Naviguer entre pages | Transition < 500ms | ☐ |
| 7.3 | Réactivité de l'éditeur | 1. Taper rapidement | Pas de lag perceptible | ☐ |
| 7.4 | Grande page | 1. Créer une page avec 100+ blocs | Édition fluide | ☐ |

---

## 8. Bugs et remarques

### Bugs trouvés

| # | Description | Sévérité | Étapes pour reproduire |
|---|-------------|----------|------------------------|
| | | | |
| | | | |
| | | | |

### Remarques / Suggestions

| # | Description | Priorité |
|---|-------------|----------|
| | | |
| | | |
| | | |

---

## Résumé

| Section | Tests passés | Tests échoués | Non testés |
|---------|:------------:|:-------------:|:----------:|
| 1. Authentification | /11 | | |
| 2. Workspaces | /4 | | |
| 3. Pages | /10 | | |
| 4. Éditeur | /35 | | |
| 5. Sauvegarde | /4 | | |
| 6. Interface | /8 | | |
| 7. Performance | /4 | | |
| **Total** | **/76** | | |

---

**Signature du testeur :** _________________________ **Date :** _____________
