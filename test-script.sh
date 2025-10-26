#!/bin/bash

# Colors for better readability
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Testing YAML Formatter HTTP Service${NC}"

# Check if the service is running
echo -e "\n${BLUE}Checking if service is running...${NC}"
HEALTH_STATUS=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:6666/health)

if [ "$HEALTH_STATUS" -eq 200 ]; then
  echo -e "${GREEN}Service is running!${NC}"
else
  echo -e "${RED}Service is not running. Please start the service first.${NC}"
  exit 1
fi

# Define input and output files
INPUT_FILE="input.txt"
OUTPUT_FILE="curl_output.yaml"

# Check if the input file exists
if [ ! -f "$INPUT_FILE" ]; then
  echo -e "${RED}Input file '$INPUT_FILE' not found.${NC}"
  exit 1
fi

echo -e "\n${BLUE}Uploading YAML file for formatting...${NC}"

# Upload the file and save the response
curl -s -X POST \
  -F "file=@$INPUT_FILE" \
  http://localhost:6666/format-yaml \
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

echo -e "\n${GREEN}Test completed successfully!${NC}"
