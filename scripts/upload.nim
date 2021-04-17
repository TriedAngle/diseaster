import tables
import strutils
import httpclient
import json
import parsecsv
from os import paramStr
from streams import newFileStream

var diseaseFilePath = "diseases.csv"
var departmentFilePath = "departments.csv"
var diseaseFile = newFileStream(diseaseFilePath, fmRead)
var departmentFile = newFileStream(departmentFilePath, fmRead)

if diseaseFile == nil and departmentFile == nil:
    quit("cannot open files")

var parser: CsvParser
parser.open(diseaseFilePath)
parser.readHeaderRow

var unique_symptoms = newSeq[string]()
var unique_diseases = newSeq[string]()

var full_diseases = initTable[string, seq[string]]()
var full_departments = initTable[string, seq[string]]()

while parser.readRow:
    var symptoms = newSeq[string]()
    var disease: string
    for col in items(parser.headers):
        if col == "symptoms":
            var char_symptom = newSeq[char]()
            var inQuotation = false
            var counter = 0
            var chars = parser.rowEntry(col)
            chars.add(',')
            for character in chars:
                counter += 1
                if character == '\'':
                    inQuotation = not inQuotation

                if not inQuotation and character == ',':
                    var symptom = newStringOfCap(len(char_symptom))
                    for val in char_symptom:
                        add(symptom, val)

                    symptoms.add(symptom)
                    char_symptom = newSeq[char]()
                    if not unique_symptoms.contains(symptom):
                        unique_symptoms.add(symptom)
                else:
                    if character != '\'':
                        char_symptom.add(character)

        if col == "diseases":
            disease = parser.rowEntry(col)
            if not unique_diseases.contains(disease):
                unique_diseases.add(disease)

    full_diseases[disease] = symptoms

parser.open(departmentFilePath)
parser.readHeaderRow

while parser.readRow:
    var diseases = newSeq[string]()
    var department: string
    for col in items(parser.headers):
        if col == "diseases":
            var char_disease = newSeq[char]()
            var inQuotation = false
            var counter = 0
            var chars = parser.rowEntry(col)
            chars.add(',')
            for character in chars:
                counter += 1
                if character == '\'':
                    inQuotation = not inQuotation

                if not inQuotation and character == ',':
                    var disease = newStringOfCap(len(char_disease))
                    for val in char_disease:
                        add(disease, val)

                    diseases.add(disease)
                else:
                    if character != '\'':
                        char_disease.add(character)

        if col == "department":
            department = parser.rowEntry(col)

    full_departments[department] = diseases

parser.close()

var client = newHttpClient()
client.headers = newHttpHeaders({"Content-Type": "application/json"})

var symptomIds = initTable[string, int]()
var diseaseIds = initTable[string, int]()
var departmentIds = initTable[string, int]()

for symptom in unique_symptoms:
    let body = %* {
      "name": symptom
    }


    let response = client.postContent("http://localhost:8080/api/symptoms", body = $body)
    let data = parseJson(response);
    symptomIds[symptom] = data["id"].getInt()

for disease, symptoms in full_diseases:
    var symptomsInIds = newSeq[int]()

    for symptom in symptoms:
        symptomsInIds.add(symptomIds[symptom])

    let body = %* {
      "name": disease,
      "symptoms": symptomsInIds
    }

    let response = client.postContent("http://localhost:8080/api/diseases", body = $body)
    let data = parseJson(response);
    diseaseIds[disease] = data["id"].getInt()


for department, diseases in full_departments:
    var diseasesInIds = newSeq[int]()

    for disease in diseases:
        diseasesInIds.add(diseaseIds[disease])

    let body = %* {
      "name": department,
      "diseases": diseasesInIds
    }

    let response = client.postContent("http://localhost:8080/api/departments", body = $body)
    let data = parseJson(response);
    departmentIds[department] = data["id"].getInt()

echo "symptoms_to_ids: ", symptomIds
echo "diseases_to_ids: ", diseaseIds
echo "department_to_ids: ", departmentIds
