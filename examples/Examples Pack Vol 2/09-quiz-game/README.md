# Quiz-Spiel

## 🎮 Beschreibung

Ein interaktives Quiz-System mit mehreren Kategorien, Schwierigkeitsgraden und Leaderboards. Dieses Beispiel zeigt:

- Session-Management für Spiele
- Multiple-Choice-Fragen
- Scoring und Bewertung
- Leaderboards
- User-Statistiken
- Admin-Funktionen
- State-Management

## 🎯 Lernziele

- Game-Logic implementieren
- Session-basierte Zustandsverwaltung
- Scoring-Algorithmen
- Datenaggregation für Statistiken
- Authentifizierung und Rollen
- Listen-Manipulation (Mischen)
- Zeitbasierte Berechnungen

## 🚀 Verwendung

### Quiz starten
```bash
POST /api/quiz/start
{
    "category": "Programmierung",
    "difficulty": "Mittel",
    "questionCount": 5
}
```

Antwort:
```json
{
    "id": "session-123",
    "userId": "user-456",
    "category": "Programmierung",
    "startedAt": "2026-01-28 15:00:00",
    "currentQuestionIndex": 0,
    "score": 0,
    "status": "active"
}
```

### Aktuelle Frage abrufen
```bash
GET /api/quiz/session-123/question
```

Antwort:
```json
{
    "id": "q1",
    "category": "Programmierung",
    "question": "Was bedeutet 'API'?",
    "options": [
        "Application Programming Interface",
        "Advanced Programming Integration",
        "Automated Process Integration",
        "Application Process Interface"
    ],
    "correctAnswer": 0,
    "difficulty": "Leicht",
    "points": 10
}
```

**Hinweis**: Die korrekte Antwort wird dem User nicht gezeigt!

### Frage beantworten
```bash
POST /api/quiz/session-123/answer
{
    "sessionId": "session-123",
    "answer": 0
}
```

Antwort: `true` (richtig) oder `false` (falsch)

### Quiz beenden
```bash
POST /api/quiz/session-123/complete
```

Antwort:
```json
{
    "session": {
        "score": 45,
        "correctAnswers": 4,
        "wrongAnswers": 1,
        "status": "completed"
    },
    "percentage": 80.0,
    "grade": "Sehr gut 👍",
    "timeTotal": 120
}
```

### Verfügbare Kategorien
```bash
GET /api/quiz/categories
```

Gibt zurück: `["Programmierung", "Geografie", "Wissenschaft"]`

### Leaderboard anzeigen
```bash
GET /api/quiz/leaderboard/Programmierung?limit=10
```

Antwort:
```json
{
    "topScores": [
        {
            "userId": "user-123",
            "username": "User-abc12345",
            "score": 150,
            "percentage": 95.0,
            "completedAt": "2026-01-28 14:30:00"
        },
        ...
    ],
    "category": "Programmierung"
}
```

### User-Statistiken
```bash
GET /api/quiz/stats
```

Antwort:
```json
{
    "totalQuizzes": 15,
    "totalScore": 680,
    "totalCorrect": 52,
    "totalWrong": 13,
    "averagePercentage": 80.0
}
```

### Neue Frage hinzufügen (Admin)
```bash
POST /api/quiz/questions
{
    "category": "Programmierung",
    "question": "Was ist Rekursion?",
    "options": [
        "Eine Schleife",
        "Eine Funktion die sich selbst aufruft",
        "Ein Datentyp",
        "Ein Design Pattern"
    ],
    "correctAnswer": 1,
    "difficulty": "Mittel"
}
```

## 💡 Wichtige Konzepte

### 1. Session-Management
Jedes Quiz ist eine Session mit:
- Eindeutiger ID
- User-Zuordnung
- Aktueller Zustand (Frage-Index, Score)
- Status (active/completed)

### 2. Scoring-System

**Punkte nach Schwierigkeit**:
- Leicht: 10 Punkte
- Mittel: 15 Punkte
- Schwer: 25 Punkte

**Bewertung nach Prozentsatz**:
- 90%+: Ausgezeichnet 🌟
- 75-89%: Sehr gut 👍
- 60-74%: Gut 😊
- 50-59%: Befriedigend 😐
- <50%: Nicht bestanden 😔

### 3. Fragen-Struktur
```velin
struct Question {
    id: string,
    category: string,
    question: string,
    options: List<string>,  // 4 Antwortmöglichkeiten
    correctAnswer: number,  // Index (0-3)
    difficulty: string,
    points: number,
}
```

### 4. Ablauf
```
1. Quiz starten → Session erstellt
2. Frage abrufen → Basierend auf currentQuestionIndex
3. Antwort senden → Prüfung, Score-Update
4. Wiederholen bis alle Fragen beantwortet
5. Quiz beenden → Ergebnis mit Note
```

### 5. Sicherheit
- `@Auth`: User muss eingeloggt sein
- `@Role("admin")`: Nur für Admins (neue Fragen)
- User-Check: Session-Zugriff nur für Besitzer

## 📊 Kategorien

Aktuell im Beispiel:
- **Programmierung**: API, Datenstrukturen, etc.
- **Geografie**: Hauptstädte, Länder, etc.
- **Wissenschaft**: Chemie, Physik, etc.

Einfach erweiterbar durch neue Fragen!

## 🔧 Erweiterungsmöglichkeiten

- **Multiplayer-Modus**: Quiz gegen andere spielen
- **Zeitlimit**: Pro Frage oder gesamt
- **Hints**: Joker wie "50:50" oder "Publikum"
- **Bilder/Videos**: Multimediale Fragen
- **Kategorien-Mix**: Fragen aus mehreren Kategorien
- **Achievements**: Badges für Meilensteine
- **Challenges**: Andere User herausfordern
- **Custom Quiz**: User können eigene Quiz erstellen
- **Streak-System**: Bonus für Antworten in Folge
- **Difficulty Progression**: Schwierigkeit steigt automatisch
- **Practice Mode**: Keine Punktzählung, nur lernen
- **Question Pool**: Riesige Fragendatenbank
- **Localization**: Mehrsprachige Fragen
- **Audio Questions**: Für Musik-Quiz

## 🎯 Gameplay-Varianten

### Sprint-Modus
- 20 Fragen, schnellstmöglich
- Zeitbonus für schnelle Antworten

### Survival-Modus
- Unbegrenzte Fragen
- 3 Leben, bei Fehler -1 Leben
- Game Over bei 0 Leben

### Championship-Modus
- Wöchentliche Turniere
- Top 10 bekommen Preise
- Spezielle schwere Fragen

### Team-Modus
- Teams von 2-4 Spielern
- Gemeinsamer Score
- Team-Chat

## 📈 Analytics

Das System trackt:
- ✅ Gesamtzahl gespielter Quizze
- ✅ Richtige/Falsche Antworten
- ✅ Durchschnittliche Erfolgsquote
- ✅ Score pro Kategorie
- ✅ Leaderboard-Position

Erweiterbar um:
- 📊 Beliebteste Kategorien
- 📊 Schwierigste Fragen
- 📊 Durchschnittliche Zeit pro Frage
- 📊 Beste Tageszeit für Scores
- 📊 Lernfortschritt über Zeit

## 🎓 Lernmodus

Ideal für:
- Schulunterricht
- Sprachlernen
- Prüfungsvorbereitung
- Firmenschulungen
- Wissensüberprüfung

Features für Bildung:
- Erklärungen zu Antworten
- Weiterführende Links
- Schwierigkeitsanpassung
- Fortschrittstracking
- Zertifikate bei Bestehen

## 🏆 Gamification

Motivation durch:
- **Points**: Score für richtige Antworten
- **Badges**: Achievements freischalten
- **Levels**: Mit Score aufsteigen
- **Leaderboards**: Gegen andere messen
- **Streaks**: Tägliche Quiz-Serie
- **Rewards**: Virtuelle Belohnungen

## ⚙️ Technische Details

### Fragen-Mischen
```velin
fn shuffleList<T>(list: List<T>): List<T> {
    // Fisher-Yates Shuffle-Algorithmus
    // Garantiert gleichmäßige Verteilung
}
```

### Prozent-Berechnung
```velin
let percentage = (correctAnswers * 100) / totalQuestions;
```

### Leaderboard-Sortierung
```velin
db.query(QuizSession)
    .where("category", "=", category)
    .orderBy("score", "DESC")
    .limit(10)
```
