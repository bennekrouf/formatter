#!/bin/bash

# Colors for better readability
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}==== YAML Formatter Docker Test ====${NC}"

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo -e "${RED}Docker is not installed. Please install Docker first.${NC}"
    exit 1
fi

# Check if docker-compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo -e "${RED}Docker Compose is not installed. Please install Docker Compose first.${NC}"
    exit 1
fi

# Define input and output files
INPUT_FILE="input.txt"
OUTPUT_FILE="docker_output.yaml"

# Check if the input file exists
if [ ! -f "$INPUT_FILE" ]; then
  echo -e "${RED}Input file '$INPUT_FILE' not found.${NC}"
  exit 1
fi

# Start the Docker services
echo -e "\n${YELLOW}Starting Docker containers...${NC}"
docker-compose up -d

# Wait for services to be ready
echo -e "\n${YELLOW}Waiting for services to be ready...${NC}"
sleep 5

# Check if Ollama is running
echo -e "\n${BLUE}Checking if Ollama is running...${NC}"
OLLAMA_STATUS=$(docker-compose exec -T ollama curl -s -o /dev/null -w "%{http_code}" http://localhost:11434/api/version)

if [ "$OLLAMA_STATUS" -eq 200 ]; then
  echo -e "${GREEN}Ollama is running!${NC}"
else
  echo -e "${RED}Ollama is not running properly.${NC}"
  echo -e "${YELLOW}Continuing anyway, but the test might fail...${NC}"
fi

# Check if the service is running
echo -e "\n${BLUE}Checking if YAML formatter service is running...${NC}"
HEALTH_STATUS=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/health)

if [ "$HEALTH_STATUS" -eq 200 ]; then
  echo -e "${GREEN}Service is running!${NC}"
else
  echo -e "${RED}Service is not running properly. Check Docker logs.${NC}"
  docker-compose logs yaml-formatter
  exit 1
fi

# Pull the required model (if not already downloaded)
echo -e "\n${BLUE}Ensuring model is available in Ollama...${NC}"
docker-compose exec -T ollama ollama pull deepseek-r1:8b

# Upload the file and save the response
echo -e "\n${BLUE}Uploading YAML file for formatting...${NC}"
curl -s -X POST \
  -F "file=@$INPUT_FILE" \
  http://localhost:8080/format-yaml \
  -o "$OUTPUT_FILE"

# Check if the output file was created
if [ -f "$OUTPUT_FILE" ]; then
  echo -e "${GREEN}File successfully formatted!${NC}"
  echo -e "Formatted output saved to ${BLUE}$OUTPUT_FILE${NC}"
  
  # Count endpoints in the output file
  ENDPOINT_COUNT=$(grep -c "text:" "$OUTPUT_FILE")
  echo -e "The output file contains ${GREEN}$ENDPOINT_COUNT${NC} endpoints"
else
  echo -e "${RED}Failed to save formatted output.${NC}"
  exit 1
fi

# Option to stop containers
read -p "Do you want to stop the Docker containers? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
  echo -e "\n${YELLOW}Stopping Docker containers...${NC}"
  docker-compose down
  echo -e "${GREEN}Docker containers stopped.${NC}"
fi

echo -e "\n${GREEN}Test completed successfully!${NC}"
