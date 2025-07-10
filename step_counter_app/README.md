# 🚶‍♂️ Step Counter App

Um aplicativo simples para contar passos, calcular distância percorrida, calorias queimadas e acompanhar o progresso para perder 20kg.

## 🎯 Funcionalidades

- ✅ Contador de passos manual
- ✅ Cálculo automático de distância (baseado no comprimento médio do passo)
- ✅ Cálculo de calorias queimadas (700 cal/hora caminhando a 5km/h)
- ✅ Acompanhamento do progresso para perder 20kg (7000 calorias = 1kg)
- ✅ Interface mobile-friendly
- ✅ Histórico de registros
- ✅ Botões de adição rápida (1k, 2k, 5k, 10k passos)

## 🏗️ Arquitetura

- **Backend**: Python + FastAPI + PostgreSQL
- **Frontend**: React.js
- **Database**: PostgreSQL
- **Containerização**: Docker + Docker Compose

## 📱 Como Usar

### Opção 1: Com Docker (Recomendado)

1. **Pré-requisitos**: Docker e Docker Compose instalados

2. **Clone e execute**:

```bash
cd step_counter_app
docker-compose up --build
```

3. **Acesse o app**:
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8000
   - Documentação da API: http://localhost:8000/docs

### Opção 2: Execução Manual

#### Backend

```bash
cd step_counter_app/backend

# Instalar dependências
pip install -r requirements.txt

# Configurar PostgreSQL (certifique-se que está rodando)
export DATABASE_URL="postgresql://postgres:password@localhost:5432/step_counter"

# Executar
python main.py
```

#### Frontend

```bash
cd step_counter_app/frontend

# Instalar dependências
npm install

# Executar
npm start
```

## 📊 Cálculos

### Distância

- **Comprimento médio do passo**: 0,762 metros (2,5 pés)
- **Fórmula**: Distância (km) = (Passos × 0,762m) ÷ 1000

### Calorias

- **Taxa de queima**: 700 calorias por hora
- **Velocidade de caminhada**: 5 km/h
- **Fórmula**: Calorias = (Distância ÷ 5km/h) × 700 cal/h

### Perda de Peso

- **Equivalência**: 7.000 calorias = 1kg de peso perdido
- **Meta**: 20kg = 140.000 calorias totais
- **Progresso**: (Calorias queimadas ÷ 140.000) × 100%

## 🎨 Interface

- **Design responsivo** para uso em celular
- **Gradientes coloridos** para uma experiência visual agradável
- **Cards informativos** com estatísticas em tempo real
- **Barra de progresso** visual para acompanhar a meta
- **Botões de adição rápida** para facilitar o uso

## 🔧 API Endpoints

- `GET /` - Health check
- `POST /steps` - Adicionar passos
- `GET /steps` - Listar registros de passos
- `GET /dashboard` - Dados do dashboard
- `DELETE /steps/{id}` - Deletar registro

## 📱 Uso Mobile

O app foi otimizado para uso em dispositivos móveis:

- Interface touch-friendly
- Botões grandes para fácil toque
- Layout responsivo
- Cores contrastantes para boa visibilidade

## 🚀 Próximas Melhorias

- [ ] Integração com APIs de fitness (Google Fit, Apple Health)
- [ ] Gráficos de progresso ao longo do tempo
- [ ] Metas personalizáveis
- [ ] Notificações de lembrete
- [ ] Exportação de dados
- [ ] Sistema de conquistas/badges

## 🛠️ Tecnologias Utilizadas

- **Backend**: FastAPI, SQLAlchemy, PostgreSQL, Uvicorn
- **Frontend**: React.js, Axios, CSS3
- **Database**: PostgreSQL
- **Containerização**: Docker, Docker Compose
- **Styling**: CSS puro com gradientes e animações

---

**Desenvolvido para ajudar você a atingir sua meta de perder 20kg! 💪**
