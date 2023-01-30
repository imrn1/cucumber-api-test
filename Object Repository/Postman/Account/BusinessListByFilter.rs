<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BusinessListByFilter</name>
   <tag></tag>
   <elementGuidId>94b00ca8-0994-4aec-988d-dcc12e34dfa3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;tanant_id\&quot;: ${tanant_id},\n  \&quot;account_type_id\&quot;: ${account_type_id},\n  \&quot;account_id\&quot;: ${account_id},\n  \&quot;account_number\&quot;: ${account_number},\n  \&quot;page_size\&quot;: ${page_size},\n  \&quot;page_index\&quot;: ${page_index},\n  \&quot;order_column\&quot;: ${order_column},\n  \&quot;order_by\&quot;: ${order_by},\n  \&quot;name\&quot;: ${name},\n  \&quot;start_date\&quot;: ${start_date},\n  \&quot;end_date\&quot;: ${end_date},\n  \&quot;access_status_id\&quot;: ${access_status_id},\n  \&quot;email\&quot;: ${email},\n  \&quot;phone_number\&quot;: ${phone_number},\n  \&quot;country_code\&quot;: ${country_code}\n}&quot;,
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
      <webElementGuid>f9ae93b2-9aa6-409c-bfc1-8e431ccf880d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>6e1cc21e-b53e-489a-9785-ab8ecc2558ab</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/BusinessListByFilter</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>7c421149-3ec1-4eb6-9508-e5a4c89555dd</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>655cf7c7-6437-4501-8d8e-6421fe650eb5</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>94d4f034-31a9-40b9-92a5-7e6b697b2da3</id>
      <masked>false</masked>
      <name>tanant_id</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>8ebaf8dd-00cd-460c-b84a-3295e5f90a67</id>
      <masked>false</masked>
      <name>account_type_id</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>83aeb6ff-a1ea-4d85-a54d-442285c8b004</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>b70eb7a3-3cd4-420f-8bb5-1221218d0eb6</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>0c16aa19-026c-4c1c-8302-f06348da414b</id>
      <masked>false</masked>
      <name>start_date</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>35d25b5f-45fd-45e2-a02c-e84df2151d36</id>
      <masked>false</masked>
      <name>end_date</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>90b9bbde-6357-4c0c-91ec-9bd58dc6608b</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>6a3a19b2-daa1-48dd-9101-8c37fb421214</id>
      <masked>false</masked>
      <name>access_status_id</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>1a920379-21f2-4a3a-b5d4-d52541923b84</id>
      <masked>false</masked>
      <name>phone_number</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>73bbb32e-8bcc-4e21-9447-c148fb91933e</id>
      <masked>false</masked>
      <name>account_id</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>d4ada042-4b06-45ea-8016-3739dfc7ca2e</id>
      <masked>false</masked>
      <name>country_code</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>81d5d3b4-7292-48bd-acaa-442e978ea11c</id>
      <masked>false</masked>
      <name>page_size</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>c0e50ab4-ec25-45c5-b1ba-f59e741f4eee</id>
      <masked>false</masked>
      <name>page_index</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>646cd5b1-87a1-4b82-9656-3d926b53c103</id>
      <masked>false</masked>
      <name>order_column</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>69eb0572-35b5-4680-a19f-f24f836266a2</id>
      <masked>false</masked>
      <name>order_by</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

GlobalVariable.responseObject = response
GlobalVariable.requestObject = request

println(&quot;BusinessListByFilter request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)

 
import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;BusinessListByFilter response body\n&quot;+json.toString(4)); // Print it with specified indentation
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
