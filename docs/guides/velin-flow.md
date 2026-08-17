# Velin Flow: Transaktionale Workflows

Velin Flow ist ein mächtiges System zur Orchestrierung komplexer, mehrstufiger Prozesse. Es wurde entwickelt, um das "Saga Pattern" einfach und typsicher in Ihre Anwendungen zu integrieren.

Das Kernproblem, das Velin Flow löst: Was passiert, wenn Schritt 3 von 5 fehlschlägt? Wie machen wir Schritt 1 und 2 sauber rückgängig?

---

## Inhaltsverzeichnis

1.  [Grundkonzepte](#1-grundkonzepte)
2.  [Flows definieren (`@Flow`)](#2-flows-definieren-flow)
3.  [Schritte und Kompensation](#3-schritte-und-kompensation)
4.  [State Management](#4-state-management)
5.  [Fehlerbehandlung und Retries](#5-fehlerbehandlung-und-retries)
6.  [Unterschied zu `@VelinPipeline`](#6-unterschied-zu-velinpipeline)

---

## 1. Grundkonzepte

Ein Flow ist eine Sequenz von Schritten, die atomar behandelt werden soll ("Alles oder Nichts"), aber zu lang für eine Datenbank-Transaktion ist (z.B. API-Aufrufe, E-Mails senden).

*   **Step:** Eine einzelne Aktion (z.B. "Geld abbuchen").
*   **Compensation:** Die Gegen-Aktion zum Step (z.B. "Geld zurückbuchen").
*   **Snapshot:** Der gespeicherte Zustand vor einem Schritt.

---

## 2. Flows definieren (`@Flow`)

Um eine Funktion als Workflow zu markieren, nutzen Sie den `@Flow`-Decorator. Das `flow`-Objekt wird automatisch in den Scope injiziert.

```velin
use flow

struct OrderContext {
    orderId: string,
    userId: string,
    amount: number
}

@Flow
fn processOrder(ctx: OrderContext) {
    flow.snapshot_input(ctx);
    
    // Schritte ausführen...
}
```

---

## 3. Schritte und Kompensation

Das Herzstück von Velin Flow ist die `flow.step`-Methode. Sie nimmt zwei Lambdas:
1.  Die eigentliche Aktion (`execute`).
2.  Die Rückabwicklung (`compensate`).

```velin
@Flow
fn bookTrip(userId: string) {

    // Schritt 1: Flug buchen
    let flight = flow.step(
        "book_flight",
        || flightService.book(userId, "Berlin", "New York"),
        |result| flightService.cancel(result.bookingId) // Kompensation
    );

    // Schritt 2: Hotel buchen
    // Wenn das hier fehlschlägt, wird automatisch der Flug storniert!
    let hotel = flow.step(
        "book_hotel",
        || hotelService.reserve(userId, "New York"),
        |result| hotelService.cancel(result.reservationId)
    );

    // Schritt 3: E-Mail senden (Keine Kompensation nötig/möglich)
    flow.step(
        "send_mail",
        || emailService.send(userId, "Reise gebucht!"),
        || log.info("Mail konnte nicht zurückgenommen werden.")
    );
}
```

---

## 4. State Management

Langlebige Flows müssen ihren Zustand persistieren, falls der Server neu startet. Velin Flow nutzt dafür im Hintergrund den konfigurierten Storage (Redis oder SQL).

```velin
// Status abfragen (z.B. von einer API)
let status = flow.get_status(flowId);

match status {
    FlowStatus.Running => log.info("Läuft noch..."),
    FlowStatus.Failed(err) => log.error("Fehler: " + err),
    FlowStatus.Completed(res) => log.info("Fertig!")
}
```

---

## 5. Fehlerbehandlung und Retries

Manchmal sind Fehler temporär (z.B. Netzwerk-Timeout). Sie können automatische Wiederholungen konfigurieren.

```velin
flow.step(
    "payment",
    || paymentGateway.charge(amount),
    |res| paymentGateway.refund(res.id)
).retry({
    attempts: 3,
    backoff: "exponential", // Warten: 1s, 2s, 4s
    delay: "1s"
});
```

Wenn alle Retries fehlschlagen, wird die `compensate`-Kette rückwärts ausgeführt.

---

## 6. Unterschied zu `@VelinPipeline`

Es ist wichtig, diese beiden Konzepte nicht zu verwechseln:

| Feature | **Velin Flow** (`@Flow`) | **Velin Pipeline** (`@VelinPipeline`) |
| :--- | :--- | :--- |
| **Ziel** | Zuverlässigkeit, Konsistenz | Performance, Durchsatz |
| **Modus** | Sequenziell (meistens) | Parallel |
| **Fehler** | Rollback / Kompensation | Abbruch |
| **Einsatz** | Bestellprozesse, Buchungen | Datenanalyse, Batch-Processing |

**Beispiel Pipeline (für Performance):**

```velin
use optimizer

@VelinPipeline
async fn loadDashboardData(userId: string) {
    // Diese 3 Aufrufe passieren gleichzeitig!
    let stats = await fetchStats(userId);
    let news = await fetchNews();
    let ads = await fetchAds();
    
    return { stats, news, ads };
}
```

Nutzen Sie `@Flow` für Geschäftslogik, die sicher sein muss, und `@VelinPipeline` für Datenabruf, der schnell sein muss.
