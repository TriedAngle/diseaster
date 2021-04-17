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
                    char_disease = newSeq[char]()
                else:
                    if character != '\'':
                        char_disease.add(character)

        if col == "departments":
            department = parser.rowEntry(col)

    full_departments[department] = diseases

parser.close()

echo "unique symptoms: ", unique_symptoms
echo "unique diseases: ", unique_diseases
echo "departments: ", full_departments

var symptom_client = newHttpClient()
symptom_client.headers = newHttpHeaders({"Content-Type": "application/json"})

var disease_client = newHttpClient()
disease_client.headers = newHttpHeaders({"Content-Type": "application/json"})

var department_client = newHttpClient()
department_client.headers = newHttpHeaders({"Content-Type": "application/json"})

var symptomIds = initTable[string, int]()
var diseaseIds = initTable[string, int]()
var departmentIds = initTable[string, int]()

for symptom in unique_symptoms:
    let body = %* {
      "name": symptom
    }

    let response = symptom_client.postContent("http://localhost:8080/api/symptoms", body = $body)
    let data = parseJson(response);
    symptomIds[symptom] = data["id"].getInt()

symptom_client.close()

for disease, symptoms in full_diseases:
    var symptomsInIds = newSeq[int]()

    for symptom in symptoms:
        symptomsInIds.add(symptomIds[symptom])

    let body_disease = %* {
      "name": disease,
      "symptoms": symptomsInIds
    }

    let response = disease_client.postContent("http://localhost:8080/api/diseases", body = $body_disease)
    let data = parseJson(response);
    diseaseIds[disease] = data["id"].getInt()

disease_client.close()

for department, diseases in full_departments:
    var diseasesInIds = newSeq[int]()

    var client = newHttpClient()
    client.headers = newHttpHeaders({"Content-Type": "application/json"})
    for disease in diseases:
      try:
        let disease: string = disease
        var address = "http://localhost:8080/api/diseases/by-name/" & disease
        address = address.replace(" ", "%20")
        echo "address: ", address
        let response = client.getContent(address)
        let data = parseJson(response);
        diseasesInIds.add(data["id"].getInt())
      except:
        echo "ignore: ", disease

    client.close()

    let department_body = %* {
      "name": department,
      "diseases": diseasesInIds
    }

    let response = department_client.postContent("http://localhost:8080/api/departments", body = $department_body)
    echo "response: ", response


department_client.close()

echo "symptoms_to_ids: ", symptomIds
echo "diseases_to_ids: ", diseaseIds
echo "department_to_ids: ", departmentIds
