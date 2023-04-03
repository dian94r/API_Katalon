<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Risk Profile</name>
   <tag></tag>
   <elementGuidId>f0200012-1491-40b3-acbe-3741852583aa</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;policyHolderName\&quot;: ${policyHolderName},\n    \&quot;alias\&quot;: ${alias},\n    \&quot;beneficiaryName\&quot;: ${beneficiaryName},\n    \&quot;policyHolderIdNumber\&quot;: ${policyHolderIdNumber},\n    \&quot;policyHolderAddress\&quot;: ${policyHolderAddress},\n    \&quot;policyHolderPhone\&quot;: ${policyHolderPhone},\n    \&quot;policyHolderDateOfBirth\&quot;: ${policyHolderDateOfBirth},\n    \&quot;policyHolderPlaceOfBirth\&quot;: ${policyHolderPlaceOfBirth},\n    \&quot;beneficiaryDateOfBirth\&quot;: ${beneficiaryDateOfBirth},\n    \&quot;nationality\&quot;: ${nationality},\n    \&quot;policyHolderJob\&quot;: ${policyHolderJob},\n    \&quot;policyHolderJobAddress\&quot;: ${policyHolderJobAddress},\n    \&quot;policyHolderJobPhone\&quot;: ${policyHolderJobPhone},\n    \&quot;policyHolderGender\&quot;: ${policyHolderGender},\n    \&quot;policyHolderMaritalStatus\&quot;: ${policyHolderMaritalStatus},\n    \&quot;averageEarnings\&quot;: ${averageEarnings},\n    \&quot;policyCoverageArea\&quot;: ${policyCoverageArea},\n    \&quot;typeOfInsurance\&quot;: ${typeOfInsurance},\n    \&quot;totalPremium\&quot;: ${totalPremium},\n    \&quot;paymentMethod\&quot;: ${paymentMethod},\n    \&quot;insuranceStartDate\&quot;: ${insuranceStartDate},\n    \&quot;insuranceObjective\&quot;: ${insuranceObjective},\n    \&quot;dataUpdated\&quot;: ${dataUpdated},\n    \&quot;insuranceProduct\&quot;: ${insuranceProduct}\n}&quot;,
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
      <webElementGuid>42c19f92-5d8d-4cae-b9b7-d50391139df2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>apiKey</name>
      <type>Main</type>
      <value>E97baCBt8SxmDcYNyd3KwbEA0giw8ElF</value>
      <webElementGuid>6c4a4eea-eb6a-4a3e-8eb6-fd103f50346d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api-automation-dev.ifg-life.id/v1/ao-fraud/workflow/retail-risk-profile</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.null</defaultValue>
      <description></description>
      <id>1286d97c-01b8-4315-b329-59b0ba2550f6</id>
      <masked>false</masked>
      <name>policyHolderName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b7c8233b-4fae-4b3e-aa7c-88eaa83fa06e</id>
      <masked>false</masked>
      <name>variable_0</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>382f0f98-1e53-4289-97a8-d879a0ec7d92</id>
      <masked>false</masked>
      <name>variable_1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a63972fa-14a4-4ce4-977e-0e0827274d8c</id>
      <masked>false</masked>
      <name>variable_2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>16d393a5-dcd8-49e7-b6d0-37c60299d28b</id>
      <masked>false</masked>
      <name>variable_3</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>83a41b3e-9569-4673-90d5-d45b03636c09</id>
      <masked>false</masked>
      <name>variable_4</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>470631f8-58b3-4622-aef6-a747e593ca69</id>
      <masked>false</masked>
      <name>variable_5</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9d64b223-06af-49c8-b5de-776480f7591c</id>
      <masked>false</masked>
      <name>variable_6</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bfbfdee4-7007-439c-b362-b4dcdbb9cfff</id>
      <masked>false</masked>
      <name>variable_7</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9bde9fdc-ca6e-4f91-bdcb-48df52bd9004</id>
      <masked>false</masked>
      <name>variable_8</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>51243c0c-b919-4342-aa44-1ca537b09926</id>
      <masked>false</masked>
      <name>variable_9</name>
   </variables>
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
