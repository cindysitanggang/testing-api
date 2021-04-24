<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>fill in the email, first name, last name, and avatar fields with invalid values</description>
   <name>CreateNewUserNegatifScenario3</name>
   <tag></tag>
   <elementGuidId>01786b13-7530-47a1-a078-0fbdc5510f1d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;email\&quot; : \&quot;ringostar.com\&quot;,\n  \t\&quot;first_name\&quot; : \&quot;Ringooooooooooooooooooooooooooooooooooooooooooooooooo\&quot;,\n  \t\&quot;last_name\&quot; : \&quot;Star{}\&quot;,\n  \t\&quot;avatar\&quot; : \&quot;avatar ringo\&quot;\n}\n&quot;,
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

WS.verifyElementPropertyValue(response, 'email', 'ringostar.com')
WS.verifyElementPropertyValue(response, 'first_name', 'Ringooooooooooooooooooooooooooooooooooooooooooooooooo')
WS.verifyElementPropertyValue(response, 'last_name', 'Star{}')
WS.verifyElementPropertyValue(response, 'avatar', 'avatar ringo')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
