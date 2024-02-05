# todos-api

A todo api written in Python using Flask and SQLAlchemy.

## Getting Started

### Prerequisites

- Python 3.6
- pipenv
- PostgreSQL
- Docker
- Docker Compose

### Installing

1. Clone the repo

```bash
git clone
```

2. Create the virtual environment

```bash
python3 -m venv venv
```

3. Install dependencies

```bash
pip3 install -r requirements.txt
```

4. Create a `.env` file in the root directory and add the following environment variables

```bash
FLASK_APP=app.py
FLASK_ENV=development
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/todos
PORT=8083
```
