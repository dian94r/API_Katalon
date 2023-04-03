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
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

RiskProfile = findTestObject('Object Repository/Testing/Risk Profile Corporate', [
('corporatePolisDataDTO'): [
	('policyHolderName') : policyHolderName,
	('businessLicenseNumber') : businessLicenseNumber,
	('lineOfBusiness') : lineOfBusiness,
	('corporateAddress') : corporateAddress,
	('placeOfBusiness') : placeOfBusiness,
	('dateOfBusiness') : dateOfBusiness,
	('formOfBusinessEntity') : formOfBusinessEntity,
	('numberOfParticipants') : numberOfParticipants,
	('premiumAmount') : premiumAmount,
	('typeOfInsurance') : typeOfInsurance,
	('productCategories') : productCategories,
	('dataUpdated') : dataUpdated,
	('areaClosedPolicy') : areaClosedPolicy,
	('paymentMethod') : paymentMethod,
	('insuranceProduct') : insuranceProduct,
	('areaClosedPolicy') : areaClosedPolicy,
],
('memberCorporateDataDTO'): [
	('policyHolderName') : policyHolderName,
	('idNumber') : idNumber,
	('address') : address,
	('phoneNumber') : phoneNumber,
	('dateOfBirth') : dateOfBirth,
	('placeOfBirth') : placeOfBirth,
	('nationality') : nationality,
	('jobs') : jobs,
	('gender') : gender,
	('statusMarriage') : statusMarriage,
	('sourceOfFunds') : sourceOfFunds,
	('averageEarnings') : averageEarnings
]
])

'Get Body request'
request_body = (RiskProfile.getHttpBody() as String)

WS.comment('request body ' + request_body)

'Verify Respon Body'
response = WS.sendRequest(RiskProfile)

WS.comment('response body ' + response.getResponseText())

if (statusCode == 201) {
	riskStatus = parsedJson.data.riskStatus
	assert riskStatus.length() > 0
	
	riskScore = parsedJson.data.riskScore
	WebUI.verifyMatch(riskScore, RiskScore, false, FailureHandling.CONTINUE_ON_FAILURE)
	
} else {
	try {
		Message = parsedJson.Message
		
		WebUI.verifyMatch(Message, ErrorMessage, false, FailureHandling.CONTINUE_ON_FAILURE)
		
	}
	catch (Exception ex) {
	}
}
