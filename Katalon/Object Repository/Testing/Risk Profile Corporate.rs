<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Risk Profile Corporate</name>
   <tag></tag>
   <elementGuidId>dd12ff9d-db40-4278-9cec-78e1435e319d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;corporatePolisDataDTO\&quot;: {\n        \&quot;policyHolderName\&quot;: ${policyHolderName},\n        \&quot;businessLicenseNumber\&quot;: ${businessLicenseNumber},\n        \&quot;lineOfBusiness\&quot;: ${lineOfBusiness},\n        \&quot;corporateAddress\&quot;: ${corporateAddress},\n        \&quot;placeOfBusiness\&quot;: ${placeOfBusiness},\n        \&quot;dateOfBusiness\&quot;: ${dateOfBusiness},\n        \&quot;formOfBusinessEntity\&quot;: ${formOfBusinessEntity},\n        \&quot;numberOfParticipants\&quot;: ${numberOfParticipants},\n        \&quot;premiumAmount\&quot;: ${premiumAmount},\n        \&quot;typeOfInsurance\&quot;: ${typeOfInsurance},\n        \&quot;productCategories\&quot;: ${productCategories},\n        \&quot;dataUpdated\&quot;: ${dataUpdated},\n        \&quot;areaClosedPolicy\&quot;: ${areaClosedPolicy},\n        \&quot;paymentMethod\&quot;: ${paymentMethod},\n        \&quot;insuranceProduct\&quot;: ${insuranceProduct}\n    },\n    \&quot;memberCorporateDataDTO\&quot;: {\n        \&quot;policyHolderName\&quot;: ${policyHolderName},\n        \&quot;idNumber\&quot;: ${idNumber},\n        \&quot;address\&quot;: ${address},\n        \&quot;phoneNumber\&quot;: ${phoneNumber},\n        \&quot;dateOfBirth\&quot;: ${dateOfBirth},\n        \&quot;placeOfBirth\&quot;: ${placeOfBirth},\n        \&quot;nationality\&quot;: ${nationality},\n        \&quot;jobs\&quot;: ${jobs},\n        \&quot;gender\&quot;: ${gender},\n        \&quot;statusMarriage\&quot;: ${statusMarriage},\n        \&quot;sourceOfFunds\&quot;: ${sourceOfFunds},\n        \&quot;averageEarnings\&quot;: ${averageEarnings}\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>76a35060-7e9b-4be4-8810-b31ca97e52e6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api-automation-uat.ifg-life.id/v1/ao-fraud/risk-profile/corporate/validate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
