import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.TestObjectProperty
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper
import internal.GlobalVariable
String body =
'{'+
'"name":"' + name + '",' +
'"job":"' + job + '",' +
'"id":"' + id +
'"}'

System.out.println(body)

//prepare the http request
def request = (RequestObject) findTestObject('Testing2/TestingAPIContoh')
request.setBodyContent(new HttpTextBodyContent(body, "UTF-8", "application/json"))
def response = WS.sendRequest(request)
//get response
def body_content = response.responseBodyContent
def respDataMap = new JsonSlurper().parseText(body_content)
//untuk liat isi dari variabel
System.out.println(respDataMap)
println (body_content)
//untuk verify salah satu parameter response
WS.verifyEqual(name, respDataMap.name)
WS.verifyEqual(201, response.statusCode)