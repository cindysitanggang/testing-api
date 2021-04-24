<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>create a user by entering a combination of numbers and special characters in the first name and last name fields</description>
   <name>CreateNewUserNegatifScenario1</name>
   <tag></tag>
   <elementGuidId>2774725e-1e2a-47fb-9f5b-12b632735358</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;email\&quot; : \&quot;jhon.lenon@reqres.in\&quot;,\n  \t\&quot;first_name\&quot; : \&quot;Jhon 12345\&quot;,\n  \t\&quot;last_name\&quot; : \&quot;Lenon !@#$%\&quot;,\n  \t\&quot;avatar\&quot; : \&quot;https://reqres.in/img/faces/1-image.jpg\&quot;\n}\n&quot;,
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
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://reqres.in/api/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)

WS.verifyElementPropertyValue(response, 'email', 'jhon.lenon@reqres.in')
WS.verifyElementPropertyValue(response, 'first_name', 'Jhon 12345')
WS.verifyElementPropertyValue(response, 'last_name', 'Lenon !@#$%')
WS.verifyElementPropertyValue(response, 'avatar', 'https://reqres.in/img/faces/1-image.jpg')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
