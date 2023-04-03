import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
//import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper as JsonSlurper
RiskProfile = findTestObject('Object Repository/Testing/Risk Profile', [ 
	('policyHolderName') : policyHolderName,
	('alias') : alias,
	('beneficiaryName') : beneficiaryName,
	('policyHolderIdNumber') : policyHolderIdNumber,
	('policyHolderAddress') : policyHolderAddress,
	('policyHolderPhone') : policyHolderPhone,
	('policyHolderDateOfBirth') : policyHolderDateOfBirth,
	('policyHolderPlaceOfBirth') : policyHolderPlaceOfBirth,
	('beneficiaryDateOfBirth') : beneficiaryDateOfBirth,
	('nationality') : nationality,
	('policyHolderJob') : policyHolderJob,
	('policyHolderJobAddress') : policyHolderJobAddress,
	('policyHolderJobPhone') : policyHolderJobPhone,
	('policyHolderGender') : policyHolderGender,
	('policyHolderMaritalStatus') : policyHolderMaritalStatus,
	('averageEarnings') : averageEarnings,
	('policyCoverageArea') : policyCoverageArea,
	('typeOfInsurance') : typeOfInsurance,
	('totalPremium') : totalPremium,
	('paymentMethod') : paymentMethod,
	('insuranceStartDate') : insuranceStartDate,
	('insuranceObjective') : insuranceObjective,
	('dataUpdated') : dataUpdated,
	('insuranceProduct') : insuranceProduct
	])
'Get Body request' 
request_body = (RiskProfile.getHttpBody() as String)

WS.comment('request body ' + request_body)

'Verify Respon Body' 
response = WS.sendRequest(RiskProfile)

WS.comment('response body ' + response.getResponseText())
JsonSlurper jsonSlurper = new JsonSlurper()
Map parsedJson = jsonSlurper.parseText(response.getResponseText())

//untuk rubah format jadi json
//def respDataMap = new JsonSlurper().parseText(response)
//println{"xxx" + statusCode}
//println{"yyy" + response.statusCode}
if (statusCode == 201) { 
	riskStatus = parsedJson.data.retailRiskProfileCalculation.riskStatus
	WebUI.verifyMatch(riskStatus, RiskStatus, false, FailureHandling.CONTINUE_ON_FAILURE)
	
//	riskScore = parsedJson.data.RiskScore
//	WebUI.verifyMatch(riskScore, RiskScore, false, FailureHandling.CONTINUE_ON_FAILURE)
//	riskScore = parsedJson.data.retailRiskProfileCalculation.riskScore 
//	WebUI.verifyMatch(riskScore, RiskScore, false, FailureHandling.CONTINUE_ON_FAILURE)
	
} else { 
	WS.verifyEqual(statusCode, response.statusCode)
//	def body_content = response.responseBodyContent
//	def respDataMap = new JsonSlurper().parseText(body_content)
//	WS.verifyEqual(ErrorMessage, respDataMap.message)

}
//	try { 
		Message = parsedJson.Message
		
		WebUI.verifyMatch(Message, ErrorMessage, false, FailureHandling.CONTINUE_ON_FAILURE)
		
//	} 
//	catch (Exception ex) { 
//	} 

	